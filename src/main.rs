// gazeOff — a quiet recovery companion for the Windows tray.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, PhysicalPosition, State, WebviewUrl, WebviewWindowBuilder,
    WindowEvent,
};

// ---------- settings ----------

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    interval_min: u32, // work minutes between short breaks
    short_secs: u32,   // short break duration
    long_every: u32,   // every Nth break is long
    long_min: u32,     // long break duration (minutes)
    mode: String,      // lenient | smart | focused
    force_due: bool,   // smart mode: unskippable break after 3 skips
    playful: bool,     // voice
    blink: bool,
    blink_min: u32,
    posture: bool,
    posture_min: u32,
    prebreak: bool,  // heads-up before breaks
    lead_secs: u32,  // how far in advance
    sound: bool,     // soft chime when a break completes
    autostart: bool, // launch with Windows
    smart_fullscreen: bool, // hold breaks during fullscreen apps
    cooldown_secs: u32,     // grace period after fullscreen ends
    idle_pause: u32,        // stop counting after N seconds away
    idle_reset: u32,        // treat N seconds away as a real break
    hours_start: u32,
    hours_end: u32,
    days: u8, // bitmask, bit 0 = Sunday
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            interval_min: 20,
            short_secs: 40,
            long_every: 3,
            long_min: 5,
            mode: "smart".into(),
            force_due: false,
            playful: false,
            blink: true,
            blink_min: 10,
            posture: true,
            posture_min: 30,
            prebreak: true,
            lead_secs: 60,
            sound: true,
            autostart: false,
            smart_fullscreen: true,
            cooldown_secs: 60,
            idle_pause: 60,
            idle_reset: 300,
            hours_start: 0,
            hours_end: 24,
            days: 0b0111_1111,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
struct Day {
    date: String,
    taken: u32,
    skipped: u32,
    longest: u32,               // longest unbroken session, seconds
    sessions: Vec<(u32, bool)>, // (minutes, ended-with-break) per session today
}

// ---------- engine ----------

#[derive(Clone)]
struct Brk {
    long: bool,
    dur: u32,
    t: u32,
}

#[derive(Default)]
struct Engine {
    s: Settings,
    day: Day,
    streak: u32, // consecutive good days
    work: u32,   // active seconds since last completed break
    blink_t: u32,
    post_t: u32,
    pending: bool,
    warned: bool,
    brk: Option<Brk>,
    returning: u32, // post-break return-moment countdown
    paused_until: u64,
    nudge_until: u64,
    fs_until: u64, // smart-pause cooldown horizon
    debt: u32,     // consecutive skipped breaks
    shorts: u32,   // short breaks since last long one
}

impl Engine {
    fn state(&self) -> &'static str {
        if self.debt >= 3 || self.work >= 100 * 60 {
            "due"
        } else if self.debt >= 1 || self.work >= 75 * 60 {
            "accumulating"
        } else {
            "clear"
        }
    }

    fn interval(&self) -> u32 {
        self.s.interval_min * 60
    }

    fn skippable(&self) -> bool {
        match self.s.mode.as_str() {
            "lenient" => true,
            "focused" => false,
            _ => !(self.s.force_due && self.debt >= 3),
        }
    }

    fn snap(&self, now: u64) -> Value {
        json!({
            "state": self.state(),
            "paused": now < self.paused_until,
            "next_in": self.interval().saturating_sub(self.work),
            "interval": self.interval(),
            "debt": self.debt,
            "playful": self.s.playful,
            "mode": self.s.mode,
            "sound": self.s.sound,
            "returning": self.returning,
            "streak": self.streak,
            "day": {
                "taken": self.day.taken, "skipped": self.day.skipped,
                "longest": self.day.longest.max(self.work),
                "sessions": self.day.sessions,
            },
            "brk": self.brk.as_ref().map(|b| json!({
                "long": b.long, "dur": b.dur, "t": b.t,
                "skippable": self.skippable(),
                "skip_at": 3u32,
                "end_at": (b.dur * 7 / 10),
            })),
        })
    }
}

struct Eng(Arc<Mutex<Engine>>);

// ---------- win32 helpers ----------

fn idle_secs() -> u64 {
    use windows_sys::Win32::System::SystemInformation::GetTickCount;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
    unsafe {
        let mut lii = LASTINPUTINFO { cbSize: 8, dwTime: 0 };
        if GetLastInputInfo(&mut lii) == 0 {
            return 0;
        }
        (GetTickCount().wrapping_sub(lii.dwTime) / 1000) as u64
    }
}

fn local_clock() -> (String, u32, u8) {
    use windows_sys::Win32::System::SystemInformation::GetLocalTime;
    unsafe {
        let mut st = std::mem::zeroed::<windows_sys::Win32::Foundation::SYSTEMTIME>();
        GetLocalTime(&mut st);
        (
            format!("{:04}-{:02}-{:02}", st.wYear, st.wMonth, st.wDay),
            st.wHour as u32,
            st.wDayOfWeek as u8,
        )
    }
}

// True when the foreground window covers its whole monitor (game, video, slides).
fn fullscreen_foreground() -> bool {
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetClassNameW, GetForegroundWindow, GetWindowRect,
    };
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return false;
        }
        let mut class = [0u16; 64];
        let n = GetClassNameW(hwnd, class.as_mut_ptr(), 64) as usize;
        let class = String::from_utf16_lossy(&class[..n]);
        if class == "Progman" || class == "WorkerW" || class == "Shell_TrayWnd" {
            return false;
        }
        let mut r = std::mem::zeroed::<windows_sys::Win32::Foundation::RECT>();
        if GetWindowRect(hwnd, &mut r) == 0 {
            return false;
        }
        let mon = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        let mut mi = std::mem::zeroed::<MONITORINFO>();
        mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        if GetMonitorInfoW(mon, &mut mi) == 0 {
            return false;
        }
        let m = mi.rcMonitor;
        r.left <= m.left && r.top <= m.top && r.right >= m.right && r.bottom >= m.bottom
    }
}

fn set_autostart(on: bool) {
    use std::os::windows::process::CommandExt;
    const NO_WINDOW: u32 = 0x0800_0000;
    let exe = std::env::current_exe().unwrap_or_default();
    let key = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
    let _ = if on {
        std::process::Command::new("reg")
            .args(["add", key, "/v", "gazeOff", "/t", "REG_SZ", "/d",
                &format!("\"{}\"", exe.display()), "/f"])
            .creation_flags(NO_WINDOW)
            .output()
    } else {
        std::process::Command::new("reg")
            .args(["delete", key, "/v", "gazeOff", "/f"])
            .creation_flags(NO_WINDOW)
            .output()
    };
}

// ---------- tray icon, drawn in code ----------

fn tray_icon(state: &str, paused: bool) -> Image<'static> {
    let (cr, cg, cb) = if paused {
        (162u8, 160u8, 156u8)
    } else {
        match state {
            "due" => (201, 130, 112),
            "accumulating" => (214, 184, 138),
            _ => (158, 182, 168),
        }
    };
    const S: i32 = 32;
    let mut px = vec![0u8; (S * S * 4) as usize];
    let c = (S as f32 - 1.0) / 2.0;
    for y in 0..S {
        for x in 0..S {
            let d = (((x as f32 - c).powi(2) + (y as f32 - c).powi(2)) as f32).sqrt();
            // soft halo ring + core dot; paused = hollow ring only
            let ring = (1.0 - ((d - 11.5).abs() - 1.2).max(0.0)).clamp(0.0, 1.0) * 0.45;
            let core = if paused {
                0.0
            } else {
                (1.0 - (d - 5.5).max(0.0)).clamp(0.0, 1.0)
            };
            let a = (ring + core).min(1.0);
            if a > 0.0 {
                let i = ((y * S + x) * 4) as usize;
                px[i] = cr;
                px[i + 1] = cg;
                px[i + 2] = cb;
                px[i + 3] = (a * 255.0) as u8;
            }
        }
    }
    Image::new_owned(px, S as u32, S as u32)
}

// ---------- persistence ----------

fn store_path(app: &AppHandle) -> std::path::PathBuf {
    let dir = app.path().app_config_dir().unwrap();
    let _ = std::fs::create_dir_all(&dir);
    dir.join("gazeoff.json")
}

fn save(app: &AppHandle, e: &Engine) {
    let v = json!({ "settings": e.s, "day": e.day, "streak": e.streak });
    let _ = std::fs::write(store_path(app), v.to_string());
}

fn load(app: &AppHandle, e: &mut Engine) {
    if let Ok(txt) = std::fs::read_to_string(store_path(app)) {
        if let Ok(v) = serde_json::from_str::<Value>(&txt) {
            if let Ok(s) = serde_json::from_value(v["settings"].clone()) {
                e.s = s;
            }
            if let Ok(d) = serde_json::from_value::<Day>(v["day"].clone()) {
                e.day = d;
            }
            e.streak = v["streak"].as_u64().unwrap_or(0) as u32;
        }
    }
}

// ---------- helpers ----------

fn now_epoch() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn show_overlay(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") {
        if let Ok(Some(mon)) = w.current_monitor().or_else(|_| app.primary_monitor()) {
            let _ = w.set_position(*mon.position());
            let _ = w.set_size(*mon.size());
        }
        let _ = w.show();
        let _ = w.set_focus();
    }
}

fn hide(app: &AppHandle, label: &str) {
    if let Some(w) = app.get_webview_window(label) {
        let _ = w.hide();
    }
}

fn show_nudge(app: &AppHandle, kind: &str, e: &mut Engine) {
    let secs = if kind == "prebreak" { 12 } else { 8 };
    e.nudge_until = now_epoch() + secs;
    if let Some(w) = app.get_webview_window("nudge") {
        if let Ok(Some(mon)) = app.primary_monitor() {
            let ms = mon.size();
            let mp = mon.position();
            let sf = mon.scale_factor();
            let (ww, wh) = ((364.0 * sf) as i32, (104.0 * sf) as i32);
            let _ = w.set_position(PhysicalPosition::new(
                mp.x + ms.width as i32 - ww - (16.0 * sf) as i32,
                mp.y + ms.height as i32 - wh - (60.0 * sf) as i32,
            ));
        }
        let _ = app.emit("nudge", json!({
            "kind": kind, "playful": e.s.playful, "state": e.state(), "secs": secs,
        }));
        let _ = w.show();
    }
}

fn start_break(app: &AppHandle, e: &mut Engine) {
    let long = e.shorts + 1 >= e.s.long_every.max(2) || e.work >= 90 * 60;
    let dur = if long { e.s.long_min * 60 } else { e.s.short_secs };
    e.day.longest = e.day.longest.max(e.work);
    e.brk = Some(Brk { long, dur, t: 0 });
    e.pending = false;
    e.warned = false;
    let _ = app.emit("snap", e.snap(now_epoch()));
    show_overlay(app);
}

fn finish_break(e: &mut Engine, taken: bool) {
    if let Some(b) = e.brk.take() {
        if e.work >= 60 {
            e.day.sessions.push((e.work / 60, taken));
            if e.day.sessions.len() > 48 {
                e.day.sessions.remove(0);
            }
        }
        if taken {
            e.day.taken += 1;
            e.debt = e.debt.saturating_sub(1);
            if b.long {
                e.shorts = 0;
            } else {
                e.shorts += 1;
            }
            e.returning = 4;
        } else {
            e.day.skipped += 1;
            e.debt += 1;
        }
    }
    e.work = if taken {
        0
    } else {
        // a skipped break comes back in five minutes, not twenty
        e.interval().saturating_sub(300)
    };
    e.blink_t = 0;
    e.post_t = 0;
}

// ---------- commands ----------

#[tauri::command]
fn snapshot(eng: State<Eng>) -> Value {
    eng.0.lock().unwrap().snap(now_epoch())
}

#[tauri::command]
fn get_settings(eng: State<Eng>) -> Settings {
    eng.0.lock().unwrap().s.clone()
}

#[tauri::command]
fn set_settings(app: AppHandle, eng: State<Eng>, s: Settings) {
    let mut e = eng.0.lock().unwrap();
    if s.autostart != e.s.autostart {
        set_autostart(s.autostart);
    }
    e.s = s;
    save(&app, &e);
    let _ = app.emit("snap", e.snap(now_epoch()));
}

#[tauri::command]
fn skip_break(app: AppHandle, eng: State<Eng>) {
    let mut e = eng.0.lock().unwrap();
    if e.brk.is_some() && e.skippable() {
        finish_break(&mut e, false);
        save(&app, &e);
        hide(&app, "overlay");
    }
}

#[tauri::command]
fn end_break(app: AppHandle, eng: State<Eng>) {
    let mut e = eng.0.lock().unwrap();
    if let Some(b) = &e.brk {
        if b.t >= b.dur * 7 / 10 {
            finish_break(&mut e, true);
            save(&app, &e);
            let _ = app.emit("snap", e.snap(now_epoch()));
        }
    }
}

#[tauri::command]
fn break_now(app: AppHandle, eng: State<Eng>) {
    let mut e = eng.0.lock().unwrap();
    if e.brk.is_none() {
        e.paused_until = 0;
        start_break(&app, &mut e);
    }
    hide(&app, "panel");
}

#[tauri::command]
fn toggle_pause(app: AppHandle, eng: State<Eng>) {
    let mut e = eng.0.lock().unwrap();
    let now = now_epoch();
    e.paused_until = if now < e.paused_until { 0 } else { now + 3600 };
    if e.brk.is_some() {
        e.brk = None;
        hide(&app, "overlay");
    }
    let _ = app.emit("snap", e.snap(now));
}

#[tauri::command]
fn open_settings(app: AppHandle) {
    hide(&app, "panel");
    if let Some(w) = app.get_webview_window("settings") {
        let _ = w.show();
        let _ = w.set_focus();
    }
}

// ---------- main ----------

fn main() {
    let engine = Arc::new(Mutex::new(Engine::default()));

    tauri::Builder::default()
        .manage(Eng(engine.clone()))
        .invoke_handler(tauri::generate_handler![
            snapshot,
            get_settings,
            set_settings,
            skip_break,
            end_break,
            break_now,
            toggle_pause,
            open_settings
        ])
        .setup(move |app| {
            let handle = app.handle().clone();
            {
                let mut e = engine.lock().unwrap();
                load(&handle, &mut e);
                let (date, _, _) = local_clock();
                if e.day.date != date {
                    e.day = Day { date, ..Default::default() };
                }
            }

            // windows (all hidden until needed)
            let overlay = WebviewWindowBuilder::new(app, "overlay", WebviewUrl::App("overlay.html".into()))
                .transparent(true)
                .decorations(false)
                .always_on_top(true)
                .skip_taskbar(true)
                .resizable(false)
                .visible(false)
                .build()?;
            let nudge = WebviewWindowBuilder::new(app, "nudge", WebviewUrl::App("nudge.html".into()))
                .transparent(true)
                .decorations(false)
                .always_on_top(true)
                .skip_taskbar(true)
                .resizable(false)
                .focused(false)
                .focusable(false)
                .shadow(false)
                .inner_size(364.0, 104.0)
                .visible(false)
                .build()?;
            let panel = WebviewWindowBuilder::new(app, "panel", WebviewUrl::App("panel.html".into()))
                .transparent(true)
                .decorations(false)
                .always_on_top(true)
                .skip_taskbar(true)
                .resizable(false)
                .inner_size(324.0, 532.0)
                .visible(false)
                .build()?;
            let settings = WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("settings.html".into()))
                .transparent(true)
                .decorations(false)
                .resizable(false)
                .inner_size(680.0, 760.0)
                .center()
                .visible(false)
                .build()?;

            // native frost
            let _ = window_vibrancy::apply_acrylic(&overlay, Some((18, 17, 15, 180)));
            let _ = window_vibrancy::apply_acrylic(&panel, Some((22, 21, 19, 190)));
            let _ = window_vibrancy::apply_acrylic(&settings, Some((22, 21, 19, 200)));
            let _ = window_vibrancy::apply_acrylic(&nudge, Some((22, 21, 19, 190)));

            // overlay & settings never truly close — they hide
            for label in ["overlay", "settings", "panel", "nudge"] {
                if let Some(w) = app.get_webview_window(label) {
                    let wc = w.clone();
                    let is_panel = label == "panel";
                    w.on_window_event(move |ev| match ev {
                        WindowEvent::CloseRequested { api, .. } => {
                            api.prevent_close();
                            let _ = wc.hide();
                        }
                        WindowEvent::Focused(false) if is_panel => {
                            let _ = wc.hide();
                        }
                        _ => {}
                    });
                }
            }

            // tray
            let m_break = MenuItem::with_id(app, "break", "Take a break now", true, None::<&str>)?;
            let m_pause = MenuItem::with_id(app, "pause", "Pause for an hour", true, None::<&str>)?;
            let m_resume = MenuItem::with_id(app, "resume", "Resume", true, None::<&str>)?;
            let m_settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let m_quit = MenuItem::with_id(app, "quit", "Quit gazeOff", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&m_break, &m_pause, &m_resume, &m_settings, &m_quit])?;

            let eng_menu = engine.clone();
            let _tray = TrayIconBuilder::with_id("main")
                .icon(tray_icon("clear", false))
                .tooltip("gazeOff")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, ev| match ev.id().as_ref() {
                    "break" => {
                        let mut e = eng_menu.lock().unwrap();
                        if e.brk.is_none() {
                            e.paused_until = 0;
                            start_break(app, &mut e);
                        }
                    }
                    "pause" => {
                        let mut e = eng_menu.lock().unwrap();
                        e.paused_until = now_epoch() + 3600;
                        if e.brk.is_some() {
                            e.brk = None;
                            hide(app, "overlay");
                        }
                    }
                    "resume" => {
                        eng_menu.lock().unwrap().paused_until = 0;
                    }
                    "settings" => {
                        if let Some(w) = app.get_webview_window("settings") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, ev| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        position,
                        ..
                    } = ev
                    {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("panel") {
                            if w.is_visible().unwrap_or(false) {
                                let _ = w.hide();
                            } else {
                                let sf = w.scale_factor().unwrap_or(1.0);
                                let (pw, ph) = ((324.0 * sf) as i32, (532.0 * sf) as i32);
                                let _ = w.set_position(PhysicalPosition::new(
                                    (position.x as i32 - pw).max(0),
                                    (position.y as i32 - ph - 12).max(0),
                                ));
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // the 1 Hz heartbeat
            let eng_tick = engine.clone();
            std::thread::spawn(move || {
                let mut last_icon = String::new();
                let mut last_tip = String::new();
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    let now = now_epoch();
                    let idle = idle_secs();
                    let mut e = eng_tick.lock().unwrap();

                    // day rollover: score yesterday, start fresh
                    let (date, hour, wday) = local_clock();
                    if e.day.date != date {
                        if !e.day.date.is_empty() {
                            if e.day.taken > 0 && e.day.taken >= e.day.skipped {
                                e.streak += 1;
                            } else if e.day.taken + e.day.skipped > 0 {
                                e.streak = 0;
                            }
                        }
                        e.day = Day { date, ..Default::default() };
                        e.debt = 0;
                        save(&handle, &e);
                    }

                    let day_off = e.s.days & (1u8 << wday.min(6)) == 0;
                    let off_hours = day_off
                        || (e.s.hours_start < e.s.hours_end
                            && (hour < e.s.hours_start || hour >= e.s.hours_end));
                    let paused = now < e.paused_until || off_hours;

                    // nudge auto-dissolve
                    if e.nudge_until != 0 && now >= e.nudge_until {
                        e.nudge_until = 0;
                        hide(&handle, "nudge");
                    }

                    if let Some(b) = &mut e.brk {
                        b.t += 1;
                        if b.t >= b.dur {
                            finish_break(&mut e, true);
                            save(&handle, &e);
                        }
                    } else if e.returning > 0 {
                        e.returning -= 1;
                        if e.returning == 0 {
                            hide(&handle, "overlay");
                        }
                    } else if !paused {
                        if idle >= e.s.idle_reset.max(120) as u64 {
                            // extended absence: the rest happened away from the desk
                            e.work = 0;
                            e.pending = false;
                            e.warned = false;
                            e.debt = 0;
                            e.blink_t = 0;
                            e.post_t = 0;
                        } else if idle < e.s.idle_pause.max(15) as u64 {
                            e.work += 1;
                            e.blink_t += 1;
                            e.post_t += 1;
                        }

                        // smart pause: a fullscreen app holds everything, plus a cooldown after
                        let fs = e.s.smart_fullscreen && fullscreen_foreground();
                        if fs {
                            e.fs_until = now + e.s.cooldown_secs as u64;
                        }

                        let iv = e.interval();
                        if e.s.prebreak && !e.warned && !e.pending && !fs
                            && e.work >= iv.saturating_sub(e.s.lead_secs)
                        {
                            e.warned = true;
                            show_nudge(&handle, "prebreak", &mut e);
                        }
                        if e.work >= iv {
                            e.pending = true;
                        }
                        // wait for a natural pause in input, and never interrupt fullscreen
                        if e.pending && idle >= 2 && now >= e.fs_until && !fs {
                            hide(&handle, "nudge");
                            e.nudge_until = 0;
                            start_break(&handle, &mut e);
                        } else if !e.pending && !fs {
                            if e.s.blink && e.blink_t >= e.s.blink_min.max(2) * 60 {
                                e.blink_t = 0;
                                show_nudge(&handle, "blink", &mut e);
                            }
                            if e.s.posture && e.post_t >= e.s.posture_min.max(5) * 60 {
                                e.post_t = 0;
                                show_nudge(&handle, "posture", &mut e);
                            }
                        }
                    }

                    // tray reflects state
                    let st = e.state().to_string();
                    let key = format!("{st}-{paused}");
                    let tip = if paused {
                        "gazeOff — paused".to_string()
                    } else if e.brk.is_some() {
                        "gazeOff — on a break".to_string()
                    } else {
                        let m = e.interval().saturating_sub(e.work) / 60;
                        format!("gazeOff — next break in {}m", m.max(1))
                    };
                    let snap = e.snap(now);
                    drop(e);

                    if let Some(tray) = handle.tray_by_id("main") {
                        if key != last_icon {
                            let _ = tray.set_icon(Some(tray_icon(&st, paused)));
                            last_icon = key;
                        }
                        if tip != last_tip {
                            let _ = tray.set_tooltip(Some(&tip));
                            last_tip = tip;
                        }
                    }
                    let _ = handle.emit("snap", snap);
                }
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building gazeOff")
        .run(|_app, event| {
            if let tauri::RunEvent::ExitRequested { api, code, .. } = event {
                if code.is_none() {
                    api.prevent_exit();
                }
            }
        });
}
