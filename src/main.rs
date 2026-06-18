// gazeOff - a quiet recovery companion for the Windows tray.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::imageops::FilterType;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, LogicalSize, Manager, PhysicalPosition, State, WebviewUrl,
    WebviewWindowBuilder, WindowEvent,
};

// ---------- settings ----------

const SETTINGS_VERSION: u64 = 1;

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    interval_secs: u32, // work seconds between short breaks
    short_secs: u32,    // short break duration
    long_every: u32,    // every Nth break is long
    long_secs: u32,     // long break duration (seconds)
    mode: String,       // lenient | smart | focused
    playful: bool,      // voice
    blink: bool,
    blink_secs: u32,
    posture: bool,
    posture_secs: u32,
    prebreak: bool,          // heads-up before breaks
    lead_secs: u32,          // how far in advance
    sound: bool,             // soft chime when a break completes
    autostart: bool,         // launch with Windows
    smart_meeting: bool,     // hold breaks while mic/camera in use (calls)
    smart_video: bool,       // hold breaks during fullscreen video playback
    smart_gaming: bool,      // hold breaks during fullscreen games
    display_off_break: bool, // use a plain black break background
    idle_pause: u32,         // stop counting after N seconds away
    idle_reset: u32,         // treat N seconds away as a real break
    hours_start: u32,
    hours_end: u32,
    days: u8,        // bitmask, bit 0 = Sunday
    bg_mode: String, // "wallpaper" | "gradient"

    // NEW FIELDS
    start_timer_on_launch: bool,
    pause_typing_dragging: bool,
    dim_reminders: bool,
    reminders_during_pauses: bool,
    reset_timers_after_break: bool,
    show_reminder_text: bool,
    bg_blur: bool,
    bg_blur_amount: u32,
    app_theme: String,
    hide_messages: bool,
    alert_position: String,
    sound_pair: String,
    sound_break_start: bool,
    sound_posture: bool,
    sound_blink: bool,
    sound_smart_pause: bool,
    sound_active_after_idle: bool,
    sound_overtime_nudge: bool,
    volume_break: u32,
    volume_reminders: u32,
    volume_nudges: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            interval_secs: 20 * 60,
            short_secs: 25,
            long_every: 3,
            long_secs: 3 * 60,
            mode: "smart".into(),
            playful: false,
            blink: true,
            blink_secs: 10 * 60,
            posture: false,
            posture_secs: 15 * 60,
            prebreak: true,
            lead_secs: 5,
            sound: true,
            autostart: false,
            smart_meeting: true,
            smart_video: true,
            smart_gaming: true,
            display_off_break: false,
            idle_pause: 120,
            idle_reset: 300,
            hours_start: 0,
            hours_end: 24,
            days: 0b0111_1111,
            bg_mode: "gradient".into(),

            start_timer_on_launch: false,
            pause_typing_dragging: false,
            dim_reminders: false,
            reminders_during_pauses: true,
            reset_timers_after_break: true,
            show_reminder_text: true,
            bg_blur: true,
            bg_blur_amount: 24,
            app_theme: "dark".into(),
            hide_messages: false,
            alert_position: "bottom_center".into(),
            sound_pair: "harp".into(),
            sound_break_start: false,
            sound_posture: false,
            sound_blink: false,
            sound_smart_pause: false,
            sound_active_after_idle: false,
            sound_overtime_nudge: false,
            volume_break: 50,
            volume_reminders: 50,
            volume_nudges: 50,
        }
    }
}

impl Settings {
    fn normalized(mut self) -> Self {
        // Generous safety rails only - the user is free to dial these in by slider
        // or by typing an exact value. We just keep them sane and non-zero.
        self.interval_secs = self.interval_secs.clamp(60, 14400);
        self.short_secs = self.short_secs.clamp(5, 600);
        self.long_every = self.long_every.clamp(2, 20);
        self.long_secs = self.long_secs.clamp(60, 7200);
        self.blink_secs = self.blink_secs.clamp(60, 10800);
        self.posture_secs = self.posture_secs.clamp(60, 14400);
        self.lead_secs = self.lead_secs.clamp(3, 600);
        self.idle_pause = self.idle_pause.clamp(15, 600);
        self.idle_reset = self.idle_reset.clamp(60, 3600);
        self.hours_start = self.hours_start.min(23);
        self.hours_end = self.hours_end.clamp(1, 24);
        self.days &= 0b0111_1111;
        if self.days == 0 {
            self.days = Settings::default().days;
        }
        if !matches!(self.mode.as_str(), "lenient" | "smart" | "focused") {
            self.mode = "smart".into();
        }
        if !matches!(self.bg_mode.as_str(), "wallpaper" | "gradient") {
            self.bg_mode = "gradient".into();
        }
        if !matches!(
            self.alert_position.as_str(),
            "cursor"
                | "top_left"
                | "top_center"
                | "top_right"
                | "left_center"
                | "center"
                | "right_center"
                | "bottom_left"
                | "bottom_center"
                | "bottom_right"
        ) {
            self.alert_position = "top_right".into();
        }
        if !matches!(
            self.sound_pair.as_str(),
            "original" | "bell" | "bubbles" | "flute" | "harp" | "piano" | "twinkle" | "whoosh"
        ) {
            self.sound_pair = "harp".into();
        }
        self.volume_break = self.volume_break.min(100);
        self.volume_reminders = self.volume_reminders.min(100);
        self.volume_nudges = self.volume_nudges.min(100);
        self.bg_blur_amount = self.bg_blur_amount.clamp(0, 48);
        self
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
struct Day {
    date: String,
    taken: u32,
    skipped: u32,
    snoozed: u32,
    snoozed_secs: u32,
    longest: u32,               // longest unbroken session, seconds
    screen_secs: u32,           // total active (non-idle, non-paused) seconds today
    break_secs: u32,            // total break time actually taken today
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
    history: Vec<Day>,  // archived past days for the stats page (rolling, capped)
    streak: u32,        // consecutive good days
    work: u32,          // scheduler seconds; skips and delays can rewind this clock
    unbroken_work: u32, // active seconds since the last completed or confirmed break
    blink_t: u32,
    post_t: u32,
    pending: bool,
    warned: bool,
    brk: Option<Brk>,
    returning: u32, // post-break return-moment countdown
    paused_until: u64,
    nudge_until: u64,
    debt: u32,           // consecutive skipped breaks
    shorts: u32,         // short breaks since last long one
    heads_up: bool,      // pre-break countdown cue is showing
    heads_up_until: u64, // countdown deadline; the break cannot open before this
    pending_since: u64,  // when work first crossed the interval (typing-aware grace)
    debt_nudged_at: u32,
    smart_paused: bool,
    schedule_paused: bool,
    pause_reason: String,
    last_delay_secs: u32,
    afk_ready: bool,
    afk_prompt_showing: bool,
    afk_idle_secs: u32,
}

impl Engine {
    fn state(&self) -> &'static str {
        let iv = self.interval().max(60);
        if self.debt >= 3 || self.work >= iv {
            "due"
        } else if self.debt >= 1 || self.work >= (iv * 3 / 4) {
            "accumulating"
        } else {
            "clear"
        }
    }

    fn interval(&self) -> u32 {
        self.s.interval_secs
    }

    fn skippable(&self) -> bool {
        match self.s.mode.as_str() {
            "lenient" => true,
            "focused" => false,
            // Smart mode tightens after repeated avoidance. Completing a real
            // break clears the debt and makes later breaks skippable again.
            _ => self.debt < 3,
        }
    }

    fn retry_after_skip(&self) -> u32 {
        match self.s.mode.as_str() {
            "lenient" => self.interval(),
            "smart" => match self.debt {
                0 | 1 => 5 * 60,
                2 => 3 * 60,
                _ => 60,
            },
            _ => 0,
        }
    }

    fn snap(&self, now: u64) -> Value {
        json!({
            "state": self.state(),
            "paused": now < self.paused_until || self.smart_paused || self.schedule_paused,
            "pause_reason": self.pause_reason,
            "next_in": self.interval().saturating_sub(self.work),
            "interval": self.interval(),
            "debt": self.debt,
            "playful": self.s.playful,
            "mode": self.s.mode,
            "sound": self.s.sound,
            "display_off_break": self.s.display_off_break,
            "bg_mode": self.s.bg_mode,
            "bg_blur": self.s.bg_blur,
            "bg_blur_amount": self.s.bg_blur_amount,
            "app_theme": self.s.app_theme,
            "hide_messages": self.s.hide_messages,
            "alert_position": self.s.alert_position.clone(),
            "returning": self.returning,
            "streak": self.streak,
            "day": {
                "taken": self.day.taken, "skipped": self.day.skipped, "snoozed": self.day.snoozed,
                "longest": self.day.longest.max(self.unbroken_work),
                "sessions": self.day.sessions,
            },
            "brk": self.brk.as_ref().map(|b| json!({
                "long": b.long, "dur": b.dur, "t": b.t,
                "skippable": self.skippable(),
                "can_delay": self.skippable(),
                "force_locked": !self.skippable(),
                "skip_at": 3u32,
                "delay_options": [60, 300],
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
        let mut lii = LASTINPUTINFO {
            cbSize: 8,
            dwTime: 0,
        };
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

#[cfg(windows)]
fn foreground_process_name() -> Option<String> {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_QUERY_LIMITED_INFORMATION,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowThreadProcessId,
    };

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return None;
        }
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, &mut pid);
        if pid == 0 {
            return None;
        }
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if process.is_null() {
            return None;
        }
        // A 1 KB path buffer is plenty for any real executable path; this runs
        // off the 1 Hz heartbeat, so we keep the stack footprint small. Longer
        // paths simply read as "unknown" (no smart-pause), which is harmless.
        let mut buf = [0u16; 1024];
        let mut len = buf.len() as u32;
        let ok = QueryFullProcessImageNameW(process, 0, buf.as_mut_ptr(), &mut len);
        CloseHandle(process);
        if ok == 0 || len == 0 {
            return None;
        }
        let path = String::from_utf16_lossy(&buf[..len as usize]);
        std::path::Path::new(&path)
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.to_ascii_lowercase())
    }
}

#[cfg(not(windows))]
fn foreground_process_name() -> Option<String> {
    None
}

// Classify the foreground app for smart-pause in one pass: a single
// fullscreen check and a single process-name lookup feed both verdicts.
// Returns (video, game).
//
// - Dedicated video players count as "video" whether windowed or fullscreen
//   (if VLC/mpv/PotPlayer is in front, you are watching something).
// - Browsers only count as "video" when fullscreen, since a normal windowed
//   browser is indistinguishable from ordinary browsing without an extension.
//   (Windowed browser video, e.g. a small YouTube window, is not yet detected;
//   that needs audio-render sensing.)
// - Any other app that owns the whole monitor is treated as a game.
fn foreground_av() -> (bool, bool) {
    let fullscreen = fullscreen_foreground();
    let name = foreground_process_name();

    let dedicated_player = matches!(
        name.as_deref(),
        Some(
            "vlc.exe"
                | "mpv.exe"
                | "potplayermini64.exe"
                | "potplayermini.exe"
                | "wmplayer.exe"
                | "video.ui.exe"
                | "netflix.exe"
                | "primevideo.exe"
        )
    );
    let browser = matches!(
        name.as_deref(),
        Some(
            "chrome.exe" | "msedge.exe" | "firefox.exe" | "brave.exe" | "opera.exe" | "vivaldi.exe"
        )
    );

    let video = dedicated_player || (fullscreen && browser);
    let game = fullscreen && !video;
    (video, game)
}

// Is the microphone or camera live right now? Windows records this in the
// CapabilityAccessManager consent store: each app key carries a
// `LastUsedTimeStop` QWORD that is 0 while the device is actively in use.
// We enumerate those keys rather than poking at audio APIs - cheap and reliable,
// and it catches Zoom, Teams, Meet, Discord, OBS, etc. uniformly.
#[cfg(windows)]
fn device_in_use(folder: &str) -> bool {
    use windows_sys::Win32::Foundation::{ERROR_NO_MORE_ITEMS, ERROR_SUCCESS};
    use windows_sys::Win32::System::Registry::{
        RegCloseKey, RegEnumKeyExW, RegGetValueW, RegOpenKeyExW, HKEY, HKEY_CURRENT_USER, KEY_READ,
        RRF_RT_REG_QWORD,
    };

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    // LastUsedTimeStop == 0 means "still in use".
    unsafe fn live(key: HKEY) -> bool {
        let name = wide("LastUsedTimeStop");
        let mut data: u64 = 1;
        let mut size = 8u32;
        let r = RegGetValueW(
            key,
            std::ptr::null(),
            name.as_ptr(),
            RRF_RT_REG_QWORD,
            std::ptr::null_mut(),
            &mut data as *mut u64 as *mut _,
            &mut size,
        );
        r == ERROR_SUCCESS && data == 0
    }

    unsafe fn enum_children_live(parent: HKEY) -> bool {
        let mut i = 0u32;
        loop {
            let mut name = [0u16; 260];
            let mut len = name.len() as u32;
            let e = RegEnumKeyExW(
                parent,
                i,
                name.as_mut_ptr(),
                &mut len,
                std::ptr::null_mut::<u32>() as _,
                std::ptr::null_mut::<u16>() as _,
                std::ptr::null_mut::<u32>() as _,
                std::ptr::null_mut::<windows_sys::Win32::Foundation::FILETIME>() as _,
            );
            if e == ERROR_NO_MORE_ITEMS {
                break;
            }
            i += 1;
            if e != ERROR_SUCCESS {
                continue;
            }
            let mut child: HKEY = std::ptr::null_mut();
            if RegOpenKeyExW(parent, name.as_ptr(), 0, KEY_READ, &mut child) == ERROR_SUCCESS {
                let hit = live(child);
                RegCloseKey(child);
                if hit {
                    return true;
                }
            }
        }
        false
    }

    unsafe {
        let base = wide(&format!(
            "Software\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\{folder}"
        ));
        let mut root: HKEY = std::ptr::null_mut();
        if RegOpenKeyExW(HKEY_CURRENT_USER, base.as_ptr(), 0, KEY_READ, &mut root) != ERROR_SUCCESS
        {
            return false;
        }
        let mut found = false;
        let mut i = 0u32;
        loop {
            let mut name = [0u16; 260];
            let mut len = name.len() as u32;
            let e = RegEnumKeyExW(
                root,
                i,
                name.as_mut_ptr(),
                &mut len,
                std::ptr::null_mut::<u32>() as _,
                std::ptr::null_mut::<u16>() as _,
                std::ptr::null_mut::<u32>() as _,
                std::ptr::null_mut::<windows_sys::Win32::Foundation::FILETIME>() as _,
            );
            if e == ERROR_NO_MORE_ITEMS {
                break;
            }
            i += 1;
            if e != ERROR_SUCCESS {
                continue;
            }
            let sub = String::from_utf16_lossy(&name[..len as usize]);
            let mut hk: HKEY = std::ptr::null_mut();
            if RegOpenKeyExW(root, name.as_ptr(), 0, KEY_READ, &mut hk) != ERROR_SUCCESS {
                continue;
            }
            // Packaged apps store the timestamp directly; desktop apps nest one
            // level deeper under "NonPackaged".
            found = if sub.eq_ignore_ascii_case("NonPackaged") {
                enum_children_live(hk)
            } else {
                live(hk)
            };
            RegCloseKey(hk);
            if found {
                break;
            }
        }
        RegCloseKey(root);
        found
    }
}

#[cfg(windows)]
fn meeting_active() -> bool {
    device_in_use("microphone") || device_in_use("webcam")
}

#[cfg(not(windows))]
fn meeting_active() -> bool {
    false
}

#[cfg(windows)]
fn cursor_pos() -> Option<(i32, i32)> {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;
    unsafe {
        let mut p = std::mem::zeroed::<POINT>();
        if GetCursorPos(&mut p) == 0 {
            None
        } else {
            Some((p.x, p.y))
        }
    }
}

#[cfg(not(windows))]
fn cursor_pos() -> Option<(i32, i32)> {
    None
}

#[cfg(windows)]
fn square_window(window: &tauri::WebviewWindow) {
    use windows_sys::Win32::Graphics::Dwm::{
        DwmSetWindowAttribute, DWMWA_WINDOW_CORNER_PREFERENCE,
    };

    // Windows 11 corner preference for frameless utility windows.
    // 1 == DWMWCP_DONOTROUND. Using the integer keeps this compiling across
    // windows-sys versions that do not expose the named constant.
    if let Ok(hwnd) = window.hwnd() {
        let pref: i32 = 1;
        unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd.0 as _,
                DWMWA_WINDOW_CORNER_PREFERENCE as u32,
                &pref as *const _ as _,
                std::mem::size_of_val(&pref) as u32,
            );
        }
    }
}

#[cfg(not(windows))]
fn square_window(_window: &tauri::WebviewWindow) {}

#[cfg(windows)]
fn current_wallpaper_data_url() -> Option<String> {
    use base64::Engine as _;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_GETDESKWALLPAPER,
    };

    let mut buf = [0u16; 32768];
    let ok = unsafe {
        SystemParametersInfoW(
            SPI_GETDESKWALLPAPER,
            buf.len() as u32,
            buf.as_mut_ptr() as _,
            0,
        )
    };
    if ok == 0 {
        return None;
    }
    let len = buf.iter().position(|c| *c == 0).unwrap_or(buf.len());
    if len == 0 {
        return None;
    }
    let path = std::path::PathBuf::from(String::from_utf16_lossy(&buf[..len]));
    let bytes = std::fs::read(&path).ok()?;
    if bytes.len() > 16 * 1024 * 1024 {
        return None;
    }
    let mime = match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "bmp" => "image/bmp",
        "webp" => "image/webp",
        _ => "image/jpeg",
    };
    Some(format!(
        "data:{};base64,{}",
        mime,
        base64::engine::general_purpose::STANDARD.encode(bytes)
    ))
}

#[cfg(not(windows))]
fn current_wallpaper_data_url() -> Option<String> {
    None
}

fn set_autostart(on: bool) {
    use std::os::windows::process::CommandExt;
    const NO_WINDOW: u32 = 0x0800_0000;
    let exe = std::env::current_exe().unwrap_or_default();
    let key = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
    let _ = if on {
        std::process::Command::new("reg")
            .args([
                "add",
                key,
                "/v",
                "gazeOff",
                "/t",
                "REG_SZ",
                "/d",
                &format!("\"{}\"", exe.display()),
                "/f",
            ])
            .creation_flags(NO_WINDOW)
            .output()
    } else {
        std::process::Command::new("reg")
            .args(["delete", key, "/v", "gazeOff", "/f"])
            .creation_flags(NO_WINDOW)
            .output()
    };
}

// ---------- display power state (lid closed / monitor off) ----------

// True when the display is powered off - lid shut on a laptop's internal
// screen, or the monitor asleep. We pause everything then: counting a "screen
// session" while the screen is off makes no sense. When docked to an external
// monitor with the lid shut, the display stays on, so the app keeps running -
// which is correct, because the user is still working.
static DISPLAY_OFF: AtomicBool = AtomicBool::new(false);

#[cfg(windows)]
unsafe extern "system" fn power_wnd_proc(
    hwnd: windows_sys::Win32::Foundation::HWND,
    msg: u32,
    wparam: windows_sys::Win32::Foundation::WPARAM,
    lparam: windows_sys::Win32::Foundation::LPARAM,
) -> windows_sys::Win32::Foundation::LRESULT {
    use windows_sys::Win32::System::Power::POWERBROADCAST_SETTING;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        DefWindowProcW, PBT_POWERSETTINGCHANGE, WM_POWERBROADCAST,
    };
    if msg == WM_POWERBROADCAST && wparam as u32 == PBT_POWERSETTINGCHANGE && lparam != 0 {
        // We only register for GUID_CONSOLE_DISPLAY_STATE, so any setting change
        // delivered here is the display state. Data[0]: 0 = off, 1 = on, 2 = dim.
        let setting = &*(lparam as *const POWERBROADCAST_SETTING);
        DISPLAY_OFF.store(setting.Data[0] == 0, Ordering::Relaxed);
        return 0;
    }
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

#[cfg(windows)]
fn spawn_display_monitor() {
    std::thread::spawn(|| unsafe {
        use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
        use windows_sys::Win32::System::Power::RegisterPowerSettingNotification;
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            CreateWindowExW, DispatchMessageW, GetMessageW, RegisterClassW,
            DEVICE_NOTIFY_WINDOW_HANDLE, HWND_MESSAGE, MSG, WNDCLASSW,
        };

        let class_name: Vec<u16> = "gazeoff_power_monitor"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let hinstance = GetModuleHandleW(std::ptr::null());

        let mut wc: WNDCLASSW = std::mem::zeroed();
        wc.lpfnWndProc = Some(power_wnd_proc);
        wc.hInstance = hinstance as _;
        wc.lpszClassName = class_name.as_ptr();
        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            std::ptr::null(),
            0,
            0,
            0,
            0,
            0,
            HWND_MESSAGE,
            std::ptr::null_mut(),
            hinstance as _,
            std::ptr::null(),
        );
        if hwnd.is_null() {
            return;
        }
        // GUID_CONSOLE_DISPLAY_STATE {6FE69556-704A-47A0-8F24-C28D936FDA47}
        // (windows-sys 0.60 does not export the named constant, so build it.)
        let display_state_guid = windows_sys::core::GUID {
            data1: 0x6FE6_9556,
            data2: 0x704A,
            data3: 0x47A0,
            data4: [0x8F, 0x24, 0xC2, 0x8D, 0x93, 0x6F, 0xDA, 0x47],
        };
        let _ = RegisterPowerSettingNotification(
            hwnd as _,
            &display_state_guid,
            DEVICE_NOTIFY_WINDOW_HANDLE,
        );

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            DispatchMessageW(&msg);
        }
    });
}

#[cfg(not(windows))]
fn spawn_display_monitor() {}

// ---------- tray icon, drawn in code ----------

static TRAY_BLOB: OnceLock<(Vec<u8>, u32, u32)> = OnceLock::new();

fn get_tray_blob() -> &'static (Vec<u8>, u32, u32) {
    TRAY_BLOB.get_or_init(|| {
        let bytes = include_bytes!("../icons/tray_blob.png");
        let img = image::load_from_memory(bytes).unwrap();
        let img = img.resize(32, 32, FilterType::Lanczos3);
        let rgba = img.to_rgba8();
        (rgba.into_raw(), img.width(), img.height())
    })
}

fn tray_icon() -> Image<'static> {
    let blob = get_tray_blob();
    Image::new_owned(blob.0.clone(), blob.1, blob.2)
}

// ---------- persistence ----------

fn store_path(app: &AppHandle) -> std::path::PathBuf {
    let dir = app.path().app_config_dir().unwrap();
    let _ = std::fs::create_dir_all(&dir);
    dir.join("gazeoff.json")
}

fn save(app: &AppHandle, e: &Engine) {
    let v = json!({
        "settings_version": SETTINGS_VERSION,
        "settings": e.s,
        "day": e.day,
        "history": e.history,
        "streak": e.streak
    });
    // Write to a temp file then rename, so a crash mid-write can't corrupt the
    // only settings/stats file. std::fs::rename replaces atomically on Windows.
    let path = store_path(app);
    let tmp = path.with_extension("json.tmp");
    if std::fs::write(&tmp, v.to_string()).is_ok() {
        let _ = std::fs::rename(&tmp, &path);
    }
}

fn settings_from_store(value: &Value) -> Settings {
    let version = value["settings_version"].as_u64().unwrap_or(0);
    if version < SETTINGS_VERSION {
        return Settings::default();
    }

    serde_json::from_value::<Settings>(value["settings"].clone())
        .unwrap_or_default()
        .normalized()
}

fn load(app: &AppHandle, e: &mut Engine) {
    if let Ok(txt) = std::fs::read_to_string(store_path(app)) {
        if let Ok(v) = serde_json::from_str::<Value>(&txt) {
            let migrated = v["settings_version"].as_u64().unwrap_or(0) < SETTINGS_VERSION;
            e.s = settings_from_store(&v);
            if let Ok(d) = serde_json::from_value::<Day>(v["day"].clone()) {
                e.day = d;
            }
            if let Ok(h) = serde_json::from_value::<Vec<Day>>(v["history"].clone()) {
                e.history = h;
            }
            e.streak = v["streak"].as_u64().unwrap_or(0) as u32;
            if migrated {
                save(app, e);
            }
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

const HISTORY_DAYS: usize = 30;

// Archive the finished day into the rolling history (only if it had activity),
// cap the window, then start a fresh day.
fn archive_day(e: &mut Engine, new_date: String) {
    if !e.day.date.is_empty() && (e.day.taken + e.day.skipped > 0 || e.day.screen_secs > 0) {
        e.history.push(e.day.clone());
        let len = e.history.len();
        if len > HISTORY_DAYS {
            e.history.drain(0..len - HISTORY_DAYS);
        }
    }
    e.day = Day {
        date: new_date,
        ..Default::default()
    };
}

fn overtime_nudge_milestone(unbroken_secs: u32, last_milestone: u32) -> Option<u32> {
    let minutes = unbroken_secs / 60;
    if minutes < 30 {
        return None;
    }
    let milestone = 30 + ((minutes - 30) / 15) * 15;
    (milestone > last_milestone).then_some(milestone)
}

fn afk_should_arm(idle: u64, reset_after: u32) -> bool {
    idle >= reset_after.max(120) as u64
}

fn afk_should_prompt(idle: u64, pause_after: u32, ready: bool, showing: bool) -> bool {
    idle < pause_after.max(15) as u64 && ready && !showing
}

fn countdown_remaining(now: u64, until: u64) -> u32 {
    until.saturating_sub(now).min(u32::MAX as u64) as u32
}

#[derive(Debug, PartialEq, Eq)]
enum DueAction {
    Wait,
    StartCountdown,
    StartBreak,
}

fn due_action(
    prebreak_enabled: bool,
    heads_up: bool,
    countdown_until: u64,
    now: u64,
    ready_after_activity: bool,
) -> DueAction {
    if heads_up {
        return if countdown_remaining(now, countdown_until) == 0 {
            DueAction::StartBreak
        } else {
            DueAction::Wait
        };
    }
    if !ready_after_activity {
        DueAction::Wait
    } else if prebreak_enabled {
        DueAction::StartCountdown
    } else {
        DueAction::StartBreak
    }
}

fn pending_break_ready(
    work: u32,
    interval: u32,
    idle: u64,
    now: u64,
    pending_since: u64,
    extend_while_typing: bool,
) -> bool {
    if work < interval {
        return false;
    }
    let max_wait = if extend_while_typing { 120 } else { 20 };
    idle >= 2 || now.saturating_sub(pending_since) >= max_wait
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

static TRACK_CURSOR: AtomicBool = AtomicBool::new(false);

fn hide(app: &AppHandle, label: &str) {
    if label == "nudge" {
        TRACK_CURSOR.store(false, Ordering::Relaxed);
    }
    if let Some(w) = app.get_webview_window(label) {
        let _ = w.hide();
    }
}

fn play_sound(app: &AppHandle, file: &str, volume: u32) {
    let payload = serde_json::json!({
        "file": file,
        "volume": volume,
    });
    let _ = app.emit("play-sound", payload);
}

#[cfg(windows)]
fn monitor_rect_from_point(x: i32, y: i32) -> Option<(i32, i32, i32, i32)> {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };
    unsafe {
        let mon = MonitorFromPoint(POINT { x, y }, MONITOR_DEFAULTTONEAREST);
        if mon.is_null() {
            return None;
        }
        let mut mi = std::mem::zeroed::<MONITORINFO>();
        mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        if GetMonitorInfoW(mon, &mut mi) == 0 {
            return None;
        }
        let r = mi.rcMonitor;
        Some((r.left, r.top, r.right, r.bottom))
    }
}

#[cfg(not(windows))]
fn monitor_rect_from_point(_x: i32, _y: i32) -> Option<(i32, i32, i32, i32)> {
    None
}

fn place_window_at(app: &AppHandle, label: &str, x: i32, y: i32, offset_x: i32, offset_y: i32) {
    if let Some(w) = app.get_webview_window(label) {
        let sf = w.scale_factor().unwrap_or(1.0);
        let Ok(size) = w.inner_size() else {
            return;
        };
        let ww = size.width as i32;
        let wh = size.height as i32;
        let margin = (14.0 * sf) as i32;
        let (mut nx, mut ny) = (
            x + (offset_x as f64 * sf) as i32,
            y + (offset_y as f64 * sf) as i32,
        );
        if let Some((left, top, right, bottom)) = monitor_rect_from_point(x, y) {
            let min_x = left + margin;
            let min_y = top + margin;
            let max_x = right - ww - margin;
            let max_y = bottom - wh - margin;
            nx = nx.clamp(min_x, max_x.max(min_x));
            ny = ny.clamp(min_y, max_y.max(min_y));
        } else if let Ok(Some(mon)) = w.current_monitor().or_else(|_| app.primary_monitor()) {
            let mp = mon.position();
            let ms = mon.size();
            let min_x = mp.x + margin;
            let min_y = mp.y + margin;
            let max_x = mp.x + ms.width as i32 - ww - margin;
            let max_y = mp.y + ms.height as i32 - wh - margin;
            nx = nx.clamp(min_x, max_x.max(min_x));
            ny = ny.clamp(min_y, max_y.max(min_y));
        }
        let _ = w.set_position(PhysicalPosition::new(nx, ny));
    }
}

fn place_nudge_at(app: &AppHandle, x: i32, y: i32) {
    // sit right beside the pointer, just below-right
    place_window_at(app, "nudge", x, y, 14, 16);
}

fn corner_position(position: &str) -> &str {
    match position {
        "top_left" | "top_right" | "bottom_left" | "bottom_right" => position,
        _ => "bottom_right",
    }
}

fn place_afk_prompt(app: &AppHandle, position: &str) {
    if let Some(w) = app.get_webview_window("afk_prompt") {
        let sf = w.scale_factor().unwrap_or(1.0);
        let Ok(size) = w.inner_size() else {
            return;
        };
        let ww = size.width as i32;
        let wh = size.height as i32;
        let margin = (24.0 * sf) as i32;
        let bottom_margin = (64.0 * sf) as i32;
        if let Ok(Some(mon)) = w.current_monitor().or_else(|_| app.primary_monitor()) {
            let mp = mon.position();
            let ms = mon.size();
            let (nx, ny) = match corner_position(position) {
                "top_left" => (mp.x + margin, mp.y + margin),
                "top_right" => (mp.x + ms.width as i32 - ww - margin, mp.y + margin),
                "bottom_left" => (mp.x + margin, mp.y + ms.height as i32 - wh - bottom_margin),
                _ => (
                    mp.x + ms.width as i32 - ww - margin,
                    mp.y + ms.height as i32 - wh - bottom_margin,
                ),
            };
            let _ = w.set_position(PhysicalPosition::new(nx, ny));
        }
    }
}

/// Place the nudge as a corner notification, just above the system tray.
fn place_nudge_anchor(app: &AppHandle, position: &str) {
    if let Some(w) = app.get_webview_window("nudge") {
        let sf = w.scale_factor().unwrap_or(1.0);
        let Ok(size) = w.inner_size() else {
            return;
        };
        let ww = size.width as i32;
        let wh = size.height as i32;
        let margin = (24.0 * sf) as i32;
        let bottom_margin = (64.0 * sf) as i32; // clear the taskbar
        if let Ok(Some(mon)) = w.current_monitor().or_else(|_| app.primary_monitor()) {
            let mp = mon.position();
            let ms = mon.size();
            let (nx, ny) = match position {
                "top_left" => (mp.x + margin, mp.y + margin),
                "top_center" => (mp.x + (ms.width as i32 - ww) / 2, mp.y + margin),
                "top_right" => (mp.x + ms.width as i32 - ww - margin, mp.y + margin),
                "left_center" => (mp.x + margin, mp.y + (ms.height as i32 - wh) / 2),
                "center" => (
                    mp.x + (ms.width as i32 - ww) / 2,
                    mp.y + (ms.height as i32 - wh) / 2,
                ),
                "right_center" => (
                    mp.x + ms.width as i32 - ww - margin,
                    mp.y + (ms.height as i32 - wh) / 2,
                ),
                "bottom_left" => (mp.x + margin, mp.y + ms.height as i32 - wh - bottom_margin),
                "bottom_center" => (
                    mp.x + (ms.width as i32 - ww) / 2,
                    mp.y + ms.height as i32 - wh - bottom_margin,
                ),
                _ => (
                    mp.x + ms.width as i32 - ww - margin,
                    mp.y + ms.height as i32 - wh - bottom_margin,
                ),
            };
            let _ = w.set_position(PhysicalPosition::new(nx, ny));
        }
    }
}

// One reminder surface, purely informational (it never takes focus, never
// intercepts clicks). Every cue stays small and appears either near the cursor
// or in the bottom-right corner. `remain` feeds the live pre-break countdown.
fn present_nudge(app: &AppHandle, e: &mut Engine, kind: &str, remain: u32) {
    e.nudge_until = match kind {
        "blink" | "posture" => now_epoch() + 3, // a glance, two breaths, gone
        "test_break" | "test_prebreak" => now_epoch() + 5,
        "debt" => now_epoch() + 4,
        _ => 0, // prebreak heads-up persists; the loop hides it when the break opens
    };

    if kind == "blink" && e.s.sound_blink {
        play_sound(app, "reminder-blink-1.wav", e.s.volume_reminders);
    } else if kind == "posture" && e.s.sound_posture {
        play_sound(app, "reminder-posture-1.wav", e.s.volume_reminders);
    } else if kind == "debt" && e.s.sound_overtime_nudge {
        play_sound(app, "notification-break-reminder.wav", e.s.volume_nudges);
    }

    let is_center_icon = kind == "blink" || kind == "posture";
    let (w, h) = if is_center_icon {
        (240.0, 240.0)
    } else {
        (240.0, 56.0)
    };

    if e.s.dim_reminders {
        if let Some(win) = app.get_webview_window("dim") {
            if let Ok(Some(mon)) = win.current_monitor().or_else(|_| app.primary_monitor()) {
                let _ = win.set_position(*mon.position());
                let _ = win.set_size(*mon.size());
            }
            let _ = win.show();
        }
    }

    if let Some(win) = app.get_webview_window("nudge") {
        let _ = win.set_size(LogicalSize::new(w, h));
        let click_through = is_center_icon || matches!(kind, "debt" | "prebreak" | "test_prebreak");
        let _ = win.set_ignore_cursor_events(click_through);

        let mut track = false;
        if kind == "prebreak" || kind == "test_prebreak" || kind == "debt" {
            track = true;
            if let Some((cx, cy)) = cursor_pos() {
                place_nudge_at(app, cx, cy);
            }
        } else if e.s.alert_position == "cursor" {
            track = true;
            if let Some((cx, cy)) = cursor_pos() {
                place_nudge_at(app, cx, cy);
            } else {
                place_nudge_anchor(app, "bottom_right");
            }
        } else {
            place_nudge_anchor(app, e.s.alert_position.as_str());
        }

        TRACK_CURSOR.store(track, Ordering::Relaxed);
        let _ = win.show();

        let _ = app.emit(
            "nudge",
            json!({
                "kind": kind,
                "playful": e.s.playful,
                "state": e.state(),
                "style": e.s.alert_position.clone(),
                "app_theme": e.s.app_theme.clone(),
                "show_reminder_text": e.s.show_reminder_text,
                "remain": remain,
                "work_mins": if kind == "debt" { e.unbroken_work / 60 } else { e.work / 60 },
            }),
        );
    }
}

fn start_break(app: &AppHandle, e: &mut Engine) {
    let long = e.shorts + 1 >= e.s.long_every.max(2) || e.unbroken_work >= 90 * 60;
    let dur = if long { e.s.long_secs } else { e.s.short_secs };
    e.day.longest = e.day.longest.max(e.unbroken_work);
    e.brk = Some(Brk { long, dur, t: 0 });
    e.pending = false;
    e.pending_since = 0;
    e.warned = false;
    e.heads_up = false;
    e.heads_up_until = 0;
    e.nudge_until = 0;

    if e.s.sound_break_start {
        let file = if e.s.sound_pair == "original" {
            "original-start.mp3"
        } else if e.s.sound_pair == "bell" {
            "bell-start.mp3"
        } else if e.s.sound_pair == "bubbles" {
            "bubbles-start.mp3"
        } else if e.s.sound_pair == "flute" {
            "flute-start.mp3"
        } else if e.s.sound_pair == "harp" {
            "harp-start.mp3"
        } else if e.s.sound_pair == "piano" {
            "piano-start.mp3"
        } else if e.s.sound_pair == "twinkle" {
            "twinkle-start.mp3"
        } else if e.s.sound_pair == "whoosh" {
            "whoosh-start.mp3"
        } else {
            "original-start.mp3"
        };
        play_sound(app, file, e.s.volume_break);
    }

    let _ = app.emit("snap", e.snap(now_epoch()));
    hide(app, "nudge");
    show_overlay(app);
}

fn finish_break(app: Option<&AppHandle>, e: &mut Engine, taken: bool) {
    if let Some(b) = e.brk.take() {
        if e.unbroken_work >= 60 {
            e.day.sessions.push((e.unbroken_work / 60, taken));
            if e.day.sessions.len() > 48 {
                e.day.sessions.remove(0);
            }
        }
        if taken {
            e.day.taken += 1;
            e.day.break_secs += b.t.min(b.dur);
            // A real break clears the debt completely. Otherwise a user who
            // skipped three times would need three perfect breaks to recover,
            // which feels punitive rather than firm.
            e.debt = 0;
            if b.long {
                e.shorts = 0;
            } else {
                e.shorts += 1;
            }
            if e.s.reset_timers_after_break {
                e.blink_t = 0;
                e.post_t = 0;
            }
            if let (true, Some(app)) = (e.s.sound, app) {
                let file = if e.s.sound_pair == "original" {
                    "original-end.mp3"
                } else if e.s.sound_pair == "bell" {
                    "bell-end.mp3"
                } else if e.s.sound_pair == "bubbles" {
                    "bubbles-end.mp3"
                } else if e.s.sound_pair == "flute" {
                    "flute-end.mp3"
                } else if e.s.sound_pair == "harp" {
                    "harp-end.mp3"
                } else if e.s.sound_pair == "piano" {
                    "piano-end.mp3"
                } else if e.s.sound_pair == "twinkle" {
                    "twinkle-end.mp3"
                } else if e.s.sound_pair == "whoosh" {
                    "whoosh-end.mp3"
                } else {
                    "original-end.mp3"
                };
                play_sound(app, file, e.s.volume_break);
            }
        } else {
            e.day.skipped += 1;
            e.debt += 1;

            // If they skip a long break, reset the counter but keep a penalty
            // (start at 1 instead of 0) so the next long break arrives sooner.
            // If they skip a short break, still increment the counter so the
            // long break isn't delayed forever.
            if b.long {
                e.shorts = 1;
            } else {
                e.shorts += 1;
            }
        }
        e.returning = 1;
        if let Some(app) = app {
            let app_clone = app.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(250));
                crate::hide(&app_clone, "overlay");
                crate::hide(&app_clone, "dim");
            });
        }
    }
    e.work = if taken {
        0
    } else {
        // Soft mode means "skip this cycle". Smart mode means "fine, but this
        // comes back soon". Focused mode never gets here because it is locked.
        let next_in = e.retry_after_skip();
        e.interval().saturating_sub(next_in)
    };
    if taken {
        e.unbroken_work = 0;
    }
    e.blink_t = 0;
    e.post_t = 0;
    e.heads_up = false;
    e.heads_up_until = 0;
    e.pending_since = 0;
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
    let s = s.normalized();
    if s.autostart != e.s.autostart {
        set_autostart(s.autostart);
    }
    if s.app_theme != e.s.app_theme {
        let theme = if s.app_theme == "light" {
            Some(tauri::Theme::Light)
        } else {
            Some(tauri::Theme::Dark)
        };
        for label in ["settings", "panel"] {
            if let Some(w) = app.get_webview_window(label) {
                let _ = w.set_theme(theme);
                #[cfg(windows)]
                {
                    let _ = window_vibrancy::clear_blur(&w);
                    let _ = window_vibrancy::apply_acrylic(&w, Some((0, 0, 0, 0)));
                }
            }
        }
    }
    e.s = s;
    save(&app, &e);
    let _ = app.emit("snap", e.snap(now_epoch()));
}

#[tauri::command]
fn reset_settings(app: AppHandle, eng: State<Eng>) -> Settings {
    let defaults = Settings::default();
    let mut e = eng.0.lock().unwrap();
    if e.s.autostart != defaults.autostart {
        set_autostart(defaults.autostart);
    }
    e.s = defaults.clone();
    save(&app, &e);

    let theme = Some(tauri::Theme::Dark);
    for label in ["settings", "panel"] {
        if let Some(window) = app.get_webview_window(label) {
            let _ = window.set_theme(theme);
        }
    }
    let _ = app.emit("snap", e.snap(now_epoch()));
    defaults
}

#[tauri::command]
fn skip_break(app: AppHandle, eng: State<Eng>) {
    let mut e = eng.0.lock().unwrap();
    if e.brk.is_some() && e.skippable() {
        finish_break(Some(&app), &mut e, false);
        save(&app, &e);
        let _ = app.emit("snap", e.snap(now_epoch()));
    } else if e.brk.is_none() {
        if e.work < 60 {
            return; // Block skip spamming immediately after a break or skip
        }
        e.work = 0;
        e.debt += 1;
        e.day.skipped += 1;
        let interval_mins = e.interval() / 60;
        e.day.sessions.push((interval_mins, false));
        save(&app, &e);
        let _ = app.emit("snap", e.snap(now_epoch()));
    }
}

#[tauri::command]
fn delay_break(app: AppHandle, eng: State<Eng>, secs: u32) {
    let mut e = eng.0.lock().unwrap();
    let additional_secs = secs.clamp(60, 300);
    if e.brk.is_some() && e.skippable() {
        e.brk = None;
        e.pending = false;
        e.pending_since = 0;
        e.heads_up = false;
        e.heads_up_until = 0;
        e.returning = 1;

        let app_clone = app.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(250));
            crate::hide(&app_clone, "overlay");
            crate::hide(&app_clone, "dim");
        });

        e.last_delay_secs = additional_secs;
        e.day.snoozed += 1;
        e.day.snoozed_secs += additional_secs;
        e.debt += 1;
        e.work = e.interval().saturating_sub(additional_secs);
        e.blink_t = 0;
        e.post_t = 0;
        save(&app, &e);
        let _ = app.emit("snap", e.snap(now_epoch()));
    } else if e.brk.is_none() {
        // Add to remaining time by reducing accumulated work
        e.work = e.work.saturating_sub(additional_secs);
        e.day.snoozed += 1;
        e.day.snoozed_secs += additional_secs;
        save(&app, &e);
        let _ = app.emit("snap", e.snap(now_epoch()));
    }
}

#[tauri::command]
fn break_now(app: AppHandle, eng: State<Eng>) {
    let mut e = eng.0.lock().unwrap();
    if e.brk.is_none() {
        e.paused_until = 0;
        e.work = e.interval();
        e.pending = true;
        e.pending_since = 0;
    }
    hide(&app, "panel");
}

#[tauri::command]
fn answer_afk_prompt(app: AppHandle, eng: State<Eng>, took_break: bool) {
    let mut e = eng.0.lock().unwrap();
    e.afk_prompt_showing = false;
    e.afk_ready = false;

    if took_break {
        if e.unbroken_work >= 60 {
            let session_mins = e.unbroken_work / 60;
            e.day.sessions.push((session_mins, true));
            if e.day.sessions.len() > 48 {
                e.day.sessions.remove(0);
            }
        }
        e.day.taken += 1;
        e.day.break_secs += e.afk_idle_secs;
        e.work = 0;
        e.unbroken_work = 0;
        e.debt = 0;
        e.debt_nudged_at = 0;
        e.pending = false;
        e.pending_since = 0;
        e.warned = false;
        e.heads_up = false;
        e.heads_up_until = 0;
        e.blink_t = 0;
        e.post_t = 0;
    } else {
        // "Still staring" - the time away was not a real break. Count the
        // overdue stretch as a skipped break so debt reflects reality. The
        // work clocks keep running (we never reset them here).
        e.debt += 1;
        e.day.skipped += 1;
    }

    e.afk_idle_secs = 0;
    save(&app, &e);
    let _ = app.emit("snap", e.snap(now_epoch()));
    hide(&app, "afk_prompt");
    hide(&app, "dim");
}

#[tauri::command]
fn toggle_pause(app: AppHandle, eng: State<Eng>) {
    let mut e = eng.0.lock().unwrap();
    let now = now_epoch();
    e.paused_until = if now < e.paused_until { 0 } else { now + 3600 };
    e.pending = false;
    e.pending_since = 0;
    e.warned = false;
    e.heads_up = false;
    e.heads_up_until = 0;
    if e.brk.is_some() {
        e.brk = None;
        hide(&app, "overlay");
    }
    hide(&app, "nudge");
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

#[tauri::command]
fn settings_window_action(app: AppHandle, action: String) {
    let Some(window) = app.get_webview_window("settings") else {
        return;
    };

    match action.as_str() {
        "minimize" => {
            let _ = window.minimize();
        }
        "maximize" => {
            if window.is_maximized().unwrap_or(false) {
                let _ = window.unmaximize();
            } else {
                let _ = window.maximize();
            }
        }
        "close" => {
            let _ = window.hide();
        }
        _ => {}
    }
}

#[tauri::command]
fn wallpaper_data_url() -> Option<String> {
    current_wallpaper_data_url()
}

#[tauri::command]
fn test_nudge(app: AppHandle, eng: State<Eng>, kind: String) {
    let mut e = eng.0.lock().unwrap();
    if kind == "break" {
        present_nudge(&app, &mut e, "test_break", 5);
    } else if kind == "test_prebreak" {
        present_nudge(&app, &mut e, "test_prebreak", 5);
    } else {
        present_nudge(&app, &mut e, &kind, 0);
    }
}

#[tauri::command]
fn test_prebreak_sequence(_app: AppHandle, eng: State<Eng>) {
    let mut e = eng.0.lock().unwrap();
    e.paused_until = 0;
    e.work = e.interval();
    e.pending = true;
    e.pending_since = 0;
}

#[tauri::command]
fn test_afk_prompt(app: AppHandle, eng: State<Eng>) {
    let mut e = eng.0.lock().unwrap();
    e.afk_prompt_showing = true;
    e.afk_ready = false;
    e.afk_idle_secs = 180;
    place_afk_prompt(&app, e.s.alert_position.as_str());
    let _ = app.emit(
        "afk-prompt",
        json!({
            "idle_secs": e.afk_idle_secs,
            "work_mins": e.unbroken_work / 60,
            "app_theme": e.s.app_theme.clone(),
        }),
    );
    if let Some(w) = app.get_webview_window("afk_prompt") {
        let _ = w.show();
        let _ = w.set_focus();
    }
}

/// Cumulative 0-100 recovery score for the day, for ambient panel display.
/// This is a daily "eye-strain ledger", not a momentary gauge: it starts at
/// 100 each morning and only falls as skips, snoozes, and overdue time
/// accumulate. Taking a break does not raise it back up - a day of strain is
/// not undone by one good break. It resets with the daily rollover.
fn recovery_score(e: &Engine) -> f32 {
    // 1. Calculate N (Total Required Breaks for a 10-hour shift)
    let interval_secs = e.s.interval_secs;
    let n = if interval_secs > 0 {
        (10.0 * 60.0 * 60.0) / (interval_secs as f32)
    } else {
        36.0
    };

    // 2. Calculate s (Effective Skips: explicit skips + ghost skips from snoozing and overdue bleed)
    let mut ghost_skips = 0.0;

    // Scale the delay penalty precisely by the exact amount of time snoozed
    if interval_secs > 0 {
        ghost_skips += (e.day.snoozed_secs as f32) / (interval_secs as f32);
    }

    if interval_secs > 0 && e.unbroken_work > interval_secs {
        let overdue_seconds = e.unbroken_work - interval_secs;
        ghost_skips += (overdue_seconds as f32) / (interval_secs as f32);
    }

    // We use the total daily skips to act as a strict cumulative biological report card.
    let s = (e.day.skipped as f32) + ghost_skips;

    // 3. Apply the Exponential Compounding Penalty
    // Score = 100 * (1 - (s / N)^k)
    let k: f32 = 0.643;

    let ratio = (s / n).clamp(0.0, 1.0);
    let penalty_factor = ratio.powf(k);

    let score = 100.0 * (1.0 - penalty_factor);

    score.clamp(0.0, 100.0)
}

#[tauri::command]
fn get_recovery_score(eng: State<Eng>) -> f32 {
    recovery_score(&eng.0.lock().unwrap())
}

/// Stats for the Studio page: today's totals, the live eye score, the streak,
/// and the last 7 days (history + today) for the bar chart.
#[tauri::command]
fn get_stats(eng: State<Eng>) -> Value {
    let e = eng.0.lock().unwrap();
    let day_json = |d: &Day| {
        json!({
            "date": d.date,
            "screen_secs": d.screen_secs,
            "break_secs": d.break_secs,
            "taken": d.taken,
            "skipped": d.skipped,
        })
    };
    let mut days: Vec<Value> = e.history.iter().map(day_json).collect();
    days.push(day_json(&e.day));
    let n = days.len();
    let days = if n > 7 { days.split_off(n - 7) } else { days };

    json!({
        "score": recovery_score(&e),
        "streak": e.streak,
        "today": {
            "screen_secs": e.day.screen_secs,
            "break_secs": e.day.break_secs,
            "taken": e.day.taken,
            "skipped": e.day.skipped,
            "longest": e.day.longest.max(e.unbroken_work),
        },
        "days": days,
    })
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
            reset_settings,
            skip_break,
            delay_break,
            break_now,
            toggle_pause,
            open_settings,
            settings_window_action,
            wallpaper_data_url,
            get_recovery_score,
            get_stats,
            answer_afk_prompt,
            test_afk_prompt,
            test_nudge,
            test_prebreak_sequence
        ])
        .setup(move |app| {
            let handle = app.handle().clone();

            // 60fps hardware cursor tracking thread for smooth pre-break texts
            let tracker_handle = handle.clone();
            std::thread::spawn(move || {
                let mut last_cx = -1;
                let mut last_cy = -1;
                let mut cached_win: Option<tauri::WebviewWindow> = None;
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(16));
                    if TRACK_CURSOR.load(Ordering::Relaxed) {
                        if cached_win.is_none() {
                            cached_win = tracker_handle.get_webview_window("nudge");
                        }
                        if let Some((cx, cy)) = cursor_pos() {
                            if cx != last_cx || cy != last_cy {
                                last_cx = cx;
                                last_cy = cy;
                                if let Some(w) = &cached_win {
                                    let _ = w.set_position(tauri::PhysicalPosition::new(cx + 14, cy + 16));
                                }
                            }
                        }
                    } else {
                        cached_win = None;
                    }
                }
            });

            let engine = handle.state::<Eng>().0.clone();
            {
                let mut e = engine.lock().unwrap();
                load(&handle, &mut e);
                if !e.s.start_timer_on_launch {
                    e.paused_until = u64::MAX;
                }
                let (date, _, _) = local_clock();
                if e.day.date != date {
                    archive_day(&mut e, date);
                }
            }

            let initial_theme = {
                let s = engine.lock().unwrap().s.app_theme.clone();
                if s == "light" { Some(tauri::Theme::Light) } else { Some(tauri::Theme::Dark) }
            };

            // windows (all hidden until needed)
            let overlay =
                WebviewWindowBuilder::new(app, "overlay", WebviewUrl::App("overlay.html".into()))
                    .transparent(true)
                    .decorations(false)
                    .always_on_top(true)
                    .skip_taskbar(true)
                    .resizable(false)
                    .shadow(false)
                    .visible(false)
                    .build()?;
            let dim =
                WebviewWindowBuilder::new(app, "dim", WebviewUrl::App("dim.html".into()))
                    .transparent(true)
                    .decorations(false)
                    .always_on_top(true)
                    .skip_taskbar(true)
                    .resizable(false)
                    .focused(false)
                    .focusable(false)
                    .shadow(false)
                    .visible(false)
                    .build()?;
            let _nudge =
                WebviewWindowBuilder::new(app, "nudge", WebviewUrl::App("nudge.html".into()))
                    .transparent(true)
                    .decorations(false)
                    .always_on_top(true)
                    .skip_taskbar(true)
                    .resizable(false)
                    .shadow(false)
                    .focused(false)
                    .focusable(false)
                    .inner_size(164.0, 42.0)
                    .visible(false)
                    .build()?;
            let afk_prompt = WebviewWindowBuilder::new(
                app,
                "afk_prompt",
                WebviewUrl::App("afk_prompt.html".into()),
            )
            .transparent(true)
            .decorations(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .resizable(false)
            .shadow(false)
            .inner_size(360.0, 138.0)
            .visible(false)
            .build()?;
            let panel =
                WebviewWindowBuilder::new(app, "panel", WebviewUrl::App("panel.html".into()))
                    .transparent(true)
                    .decorations(false)
                    .always_on_top(true)
                    .skip_taskbar(true)
                    .resizable(false)
                    .theme(initial_theme)
                    .inner_size(280.0, 320.0)
                    .visible(false)
                    .build()?;
            let settings =
                WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("settings.html".into()))
                    .transparent(true)
                    .decorations(false)
                    .resizable(true)
                    .theme(initial_theme)
                    .title("GazeOff")
                    .inner_size(1120.0, 760.0)
                    .center()
                    .visible(false)
                    .build()?;
            square_window(&overlay);
            square_window(&dim);
            square_window(&afk_prompt);
            square_window(&panel);
            square_window(&settings);

            // Apply native Windows 11 Acrylic effect for rich blur
            #[cfg(windows)]
            {
                let _ = window_vibrancy::apply_acrylic(&panel, Some((0, 0, 0, 0)));
                let _ = window_vibrancy::apply_acrylic(&settings, Some((0, 0, 0, 0)));
            }

            // overlay and settings never truly close - they hide
            for label in ["overlay", "settings", "panel", "nudge", "dim", "afk_prompt"] {
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
            let m_test_blink = MenuItem::with_id(app, "test_blink", "Test Blink Nudge", true, None::<&str>)?;
            let m_test_post = MenuItem::with_id(app, "test_posture", "Test Posture Nudge", true, None::<&str>)?;
            let m_pause = MenuItem::with_id(app, "pause", "Pause for an hour", true, None::<&str>)?;
            let m_resume = MenuItem::with_id(app, "resume", "Resume", true, None::<&str>)?;
            let m_settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let m_quit = MenuItem::with_id(app, "quit", "Quit gazeOff", true, None::<&str>)?;
            let menu =
                Menu::with_items(app, &[&m_break, &m_test_blink, &m_test_post, &m_pause, &m_resume, &m_settings, &m_quit])?;

            let eng_menu = engine.clone();
            let eng_tray = engine.clone();
            let _tray = TrayIconBuilder::with_id("main")
                .icon(tray_icon())
                .tooltip("gazeOff")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, ev| match ev.id().as_ref() {
                    "break" => {
                        let mut e = eng_menu.lock().unwrap();
                        if e.brk.is_none() {
                            e.paused_until = 0;
                            e.work = e.interval();
                            e.pending = true;
                            e.pending_since = 0;
                        }
                    }
                    "test_blink" => {
                        let mut e = eng_menu.lock().unwrap();
                        present_nudge(app, &mut e, "blink", 0);
                    }
                    "test_posture" => {
                        let mut e = eng_menu.lock().unwrap();
                        present_nudge(app, &mut e, "posture", 0);
                    }
                    "pause" => {
                        let mut e = eng_menu.lock().unwrap();
                        e.paused_until = now_epoch() + 3600;
                        e.pending = false;
    e.pending_since = 0;
    e.warned = false;
    e.heads_up = false;
    e.heads_up_until = 0;
                        if e.brk.is_some() {
                            e.brk = None;
                            hide(app, "overlay");
                            hide(app, "dim");
                        }
                        hide(app, "nudge");
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
                .on_tray_icon_event(move |tray, ev| {
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
                                let (pw, ph) = ((280.0 * sf) as i32, (320.0 * sf) as i32);
                                let _ = w.set_position(PhysicalPosition::new(
                                    (position.x as i32 - pw).max(0),
                                    (position.y as i32 - ph - 12).max(0),
                                ));

                                let is_light = eng_tray.lock().unwrap().s.app_theme == "light";
                                let theme = if is_light { Some(tauri::Theme::Light) } else { Some(tauri::Theme::Dark) };
                                let _ = w.set_theme(theme);

                                let _ = w.show();

                                #[cfg(windows)]
                                {
                                    let _ = window_vibrancy::clear_blur(&w);
                                    let _ = window_vibrancy::apply_acrylic(&w, Some((0, 0, 0, 0)));
                                }

                                let _ = w.set_focus();
                                let theme_str = if is_light { "light" } else { "dark" };
                                let _ = w.eval(format!("document.documentElement.setAttribute('data-theme', '{}'); document.body.setAttribute('data-theme', '{}');", theme_str, theme_str));
                                let _ = w.eval("if(typeof render === 'function' && window.__TAURI__ && window.__TAURI__.core) { window.__TAURI__.core.invoke('snapshot').then(render).catch(()=>{}); }");
                            }
                        }
                    }
                })
                .build(app)?;

            // Cursor-following countdown. Throttled so the tiny nudge feels
            // attached to the pointer without burning a 60fps native move loop.
            let follow_handle = handle.clone();
            let eng_follow = engine.clone();
            std::thread::spawn(move || {
                let mut last: Option<(i32, i32)> = None;
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(33));
                    let (active, follows_cursor) = {
                        let e = eng_follow.lock().unwrap();
                        (e.heads_up && e.brk.is_none(), e.s.alert_position == "cursor")
                    };
                    if active && follows_cursor {
                        if let Some((x, y)) = cursor_pos() {
                            let moved = last
                                .map(|(lx, ly)| (x - lx).abs() + (y - ly).abs() > 3)
                                .unwrap_or(true);
                            if moved {
                                place_nudge_at(&follow_handle, x, y);
                                last = Some((x, y));
                            }
                        }
                    } else {
                        last = None;
                    }
                }
            });

            // watch for the display turning off (lid shut / monitor asleep)
            spawn_display_monitor();

            // the 1 Hz heartbeat
            let eng_tick = engine.clone();
            std::thread::spawn(move || {
                let mut last_tip = String::new();
                let mut last_smart_paused = false;
                let mut last_idle_state = false;
                let mut last_cursor = cursor_pos();
                // Smart-pause sensors (fullscreen/process scan + the registry walk
                // for mic/camera) are the only heavy win32 calls here, so we run
                // them at most every 2 s and reuse the verdict on the off ticks.
                let mut sense_age: u32 = 0;
                let mut cached_video = false;
                let mut cached_game = false;
                let mut cached_meeting = false;
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    let now = now_epoch();
                    let idle = idle_secs();
                    let cur = cursor_pos();
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
                        archive_day(&mut e, date);
                        e.debt = 0;
                        e.debt_nudged_at = 0;
                        save(&handle, &e);
                    }

                    let display_off = DISPLAY_OFF.load(Ordering::Relaxed);
                    let day_off = e.s.days & (1u8 << wday.min(6)) == 0;
                    let off_hours = day_off
                        || (e.s.hours_start < e.s.hours_end
                            && (hour < e.s.hours_start || hour >= e.s.hours_end));
                    // Treat a dark display like a schedule pause so the tray shows
                    // it as resting and counting/reminders stop.
                    e.schedule_paused = off_hours || display_off;
                    e.smart_paused = false;
                    if now < e.paused_until {
                        e.pause_reason = "manual pause".into();
                    } else if display_off {
                        e.pause_reason = "display off".into();
                    } else if off_hours {
                        e.pause_reason = "outside active hours".into();
                    } else {
                        e.pause_reason.clear();
                    }
                    let paused = now < e.paused_until || off_hours || display_off;

                    // nudge auto-dissolve (transient cues only - the heads-up persists)
                    if e.nudge_until != 0 && !e.heads_up {
                        if let (Some((lx, ly)), Some((x, y))) = (last_cursor, cur) {
                            let dx = x - lx;
                            let dy = y - ly;
                            if dx * dx + dy * dy > 240 * 240 {
                                e.nudge_until = 0;
                                hide(&handle, "nudge");
                            }
                        }
                    }
                    if e.nudge_until != 0 && now >= e.nudge_until {
                        e.nudge_until = 0;
                        hide(&handle, "nudge");
                        hide(&handle, "dim");
                    }

                    if let Some(b) = &mut e.brk {
                        b.t += 1;
                        if b.t >= b.dur {
                            finish_break(Some(&handle), &mut e, true);
                            save(&handle, &e);
                            hide(&handle, "dim");
                        }
                    } else if e.returning > 0 {
                        e.returning -= 1;
                        if e.returning == 0 {
                            hide(&handle, "overlay");
                            hide(&handle, "dim");
                        }
                    } else if !paused {
                        // Smart pause is checked before accounting time. A full-screen
                        // game/video or an active call should not accrue break debt.
                        // The sensors are refreshed at most every 2 s (see above);
                        // the gating toggles are applied fresh every tick so flipping
                        // a setting takes effect immediately.
                        if sense_age == 0 {
                            let (v, g) = foreground_av();
                            cached_video = v;
                            cached_game = g;
                            cached_meeting = meeting_active();
                        }
                        sense_age = (sense_age + 1) % 2;
                        let video_now = e.s.smart_video && cached_video;
                        let gaming_now = e.s.smart_gaming && cached_game;
                        let meeting_now = e.s.smart_meeting && cached_meeting;
                        let blocked = video_now || gaming_now || meeting_now;
                        e.smart_paused = blocked;
                        if blocked && !last_smart_paused && e.s.sound_smart_pause {
                            play_sound(&handle, "notification-idle-time.wav", e.s.volume_reminders);
                        }
                        last_smart_paused = blocked;
                        if video_now {
                            e.pause_reason = "video playback".into();
                        } else if gaming_now {
                            e.pause_reason = "game".into();
                        } else if meeting_now {
                            e.pause_reason = "meeting or call".into();
                        } else {
                            e.pause_reason.clear();
                        }

                        if blocked {
                            if e.heads_up {
                                e.heads_up = false;
                                e.heads_up_until = 0;
                                e.nudge_until = 0;
                                hide(&handle, "nudge");
                            }
                        } else {
                            // time accounting
                            let is_idle = idle >= e.s.idle_pause.max(15) as u64;
                            if !is_idle && last_idle_state && e.s.sound_active_after_idle {
                                play_sound(&handle, "notification-high-engagement-activity.wav", e.s.volume_reminders);
                            }
                            last_idle_state = is_idle;

                            if afk_should_arm(idle, e.s.idle_reset) {
                                e.afk_ready = true;
                                e.afk_idle_secs = idle.min(u32::MAX as u64) as u32;
                                if e.heads_up {
                                    e.heads_up = false;
                                    e.heads_up_until = 0;
                                    hide(&handle, "nudge");
                                }
                            } else if afk_should_prompt(
                                idle,
                                e.s.idle_pause,
                                e.afk_ready,
                                e.afk_prompt_showing,
                            ) {
                                e.afk_prompt_showing = true;
                                e.afk_ready = false;
                                e.pending = false;
                                e.pending_since = 0;
                                e.warned = false;
                                if e.heads_up {
                                    e.heads_up = false;
                                    e.heads_up_until = 0;
                                    hide(&handle, "nudge");
                                }
                                place_afk_prompt(&handle, e.s.alert_position.as_str());
                                let _ = handle.emit(
                                    "afk-prompt",
                                    json!({
                                        "idle_secs": e.afk_idle_secs,
                                        "work_mins": e.unbroken_work / 60,
                                        "app_theme": e.s.app_theme.clone(),
                                    }),
                                );
                                if let Some(w) = handle.get_webview_window("afk_prompt") {
                                    let _ = w.show();
                                    let _ = w.set_focus();
                                }
                            } else if e.afk_prompt_showing {
                                // Wait for the user to classify the away period.
                            } else if idle < e.s.idle_pause.max(15) as u64 {
                                e.work += 1;
                                e.unbroken_work += 1;
                                e.blink_t += 1;
                                e.post_t += 1;
                                e.day.screen_secs += 1;
                            }

                            let iv = e.interval();
                            let lead = e.s.lead_secs;

                            // Once due, first wait for Smart's natural pause, then run a
                            // complete countdown. The overlay cannot open until 5..1 has
                            // had its full screen time.
                            if e.work >= iv {
                                if !e.pending {
                                    e.pending = true;
                                    e.pending_since = now;
                                }
                                let ready_after_activity = pending_break_ready(
                                    e.work,
                                    iv,
                                    idle,
                                    now,
                                    e.pending_since,
                                    e.s.pause_typing_dragging,
                                );
                                match due_action(
                                    e.s.prebreak,
                                    e.heads_up,
                                    e.heads_up_until,
                                    now,
                                    ready_after_activity,
                                ) {
                                    DueAction::Wait => {
                                        if e.heads_up {
                                            let remain = countdown_remaining(now, e.heads_up_until);
                                            present_nudge(&handle, &mut e, "prebreak", remain);
                                        }
                                    }
                                    DueAction::StartCountdown => {
                                        e.heads_up = true;
                                        e.heads_up_until = now + lead as u64;
                                        present_nudge(&handle, &mut e, "prebreak", lead);
                                    }
                                    DueAction::StartBreak => {
                                        e.heads_up = false;
                                        e.heads_up_until = 0;
                                        e.nudge_until = 0;
                                        hide(&handle, "nudge");
                                        start_break(&handle, &mut e);
                                    }
                                }
                            } else {
                                if e.heads_up {
                                    e.heads_up = false;
                                    e.heads_up_until = 0;
                                    e.nudge_until = 0;
                                    hide(&handle, "nudge");
                                }
                                // One tiny surface handles all non-break nudges.
                                if let Some(milestone) = (e.debt > 0)
                                    .then(|| {
                                        overtime_nudge_milestone(
                                            e.unbroken_work,
                                            e.debt_nudged_at,
                                        )
                                    })
                                    .flatten()
                                {
                                    e.debt_nudged_at = milestone;
                                    present_nudge(&handle, &mut e, "debt", 0);
                                } else if e.nudge_until <= now_epoch() {
                                    if e.s.blink && e.blink_t >= e.s.blink_secs {
                                        e.blink_t = 0;
                                        present_nudge(&handle, &mut e, "blink", 0);
                                    } else if e.s.posture && e.post_t >= e.s.posture_secs {
                                        e.post_t = 0;
                                        present_nudge(&handle, &mut e, "posture", 0);
                                    } else if e.nudge_until > 0 {
                                        e.nudge_until = 0;
                                        hide(&handle, "nudge");
                                    }
                                }
                            }
                        }
                    } else if e.heads_up {
                        // paused / off-hours: clear any lingering heads-up
                        e.heads_up = false;
                        e.heads_up_until = 0;
                        e.nudge_until = 0;
                        hide(&handle, "nudge");
                        hide(&handle, "dim");
                    }

                    // Reminders during a "smart" pause (video/game/meeting) only.
                    // A manual pause or off-hours means the user has deliberately
                    // switched the app off, so we stay quiet then.
                    if e.s.reminders_during_pauses && e.brk.is_none() && e.smart_paused {
                        let is_idle = idle >= e.s.idle_pause.max(15) as u64;
                        if !is_idle {
                            e.blink_t += 1;
                            e.post_t += 1;
                            if e.nudge_until <= now_epoch() {
                                if e.s.blink && e.blink_t >= e.s.blink_secs {
                                    e.blink_t = 0;
                                    present_nudge(&handle, &mut e, "blink", 0);
                                } else if e.s.posture && e.post_t >= e.s.posture_secs {
                                    e.post_t = 0;
                                    present_nudge(&handle, &mut e, "posture", 0);
                                } else if e.nudge_until > 0 {
                                    e.nudge_until = 0;
                                    hide(&handle, "nudge");
                                }
                            }
                        }
                    }

                    // tray tooltip reflects state (the icon itself is static)
                    let icon_paused = now < e.paused_until || e.schedule_paused || e.smart_paused;
                    let tip = if icon_paused {
                        if e.pause_reason.is_empty() {
                            "gazeOff - resting".to_string()
                        } else {
                            format!("gazeOff - paused: {}", e.pause_reason)
                        }
                    } else if e.brk.is_some() {
                        "gazeOff - break in progress".to_string()
                    } else {
                        let s = e.interval().saturating_sub(e.work);
                        if s < 60 {
                            format!("gazeOff - break in {}s", s)
                        } else {
                            format!("gazeOff - break in {}m", (s as f32 / 60.0).ceil() as u32)
                        }
                    };
                    let snap = e.snap(now);
                    drop(e);
                    last_cursor = cur;

                    if let Some(tray) = handle.tray_by_id("main") {
                        if tip != last_tip {
                            let _ = tray.set_tooltip(Some(&tip));
                            last_tip = tip;
                        }
                        let _ = handle.emit("snap", snap);
                    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recommended_defaults_match_the_settings_page() {
        let settings = Settings::default();
        assert_eq!(settings.short_secs, 25);
        assert_eq!(settings.long_secs, 3 * 60);
        assert_eq!(settings.mode, "smart");
        assert_eq!(settings.idle_pause, 2 * 60);
        assert_eq!(settings.posture_secs, 15 * 60);
        assert_eq!(settings.blink_secs, 10 * 60);
        assert_eq!(settings.alert_position, "bottom_center");
        assert!(settings.reminders_during_pauses);
        assert!(settings.show_reminder_text);
        assert!(!settings.display_off_break);
    }

    #[test]
    fn legacy_saved_settings_migrate_once_to_new_defaults() {
        let legacy = json!({
            "settings": {
                "short_secs": 20,
                "blink_secs": 60,
                "alert_position": "center"
            }
        });

        let settings = settings_from_store(&legacy);
        assert_eq!(settings.short_secs, 25);
        assert_eq!(settings.blink_secs, 10 * 60);
        assert_eq!(settings.alert_position, "bottom_center");
    }

    #[test]
    fn smart_mode_escalates_retries_and_then_locks() {
        let mut engine = Engine::default();
        engine.s.mode = "smart".into();

        engine.debt = 1;
        assert!(engine.skippable());
        assert_eq!(engine.retry_after_skip(), 5 * 60);

        engine.debt = 2;
        assert!(engine.skippable());
        assert_eq!(engine.retry_after_skip(), 3 * 60);

        engine.debt = 3;
        assert!(!engine.skippable());
        assert_eq!(engine.retry_after_skip(), 60);
    }

    #[test]
    fn overtime_nudges_only_fire_on_fifteen_minute_milestones() {
        assert_eq!(overtime_nudge_milestone(29 * 60 + 59, 0), None);
        assert_eq!(overtime_nudge_milestone(30 * 60, 0), Some(30));
        assert_eq!(overtime_nudge_milestone(44 * 60, 30), None);
        assert_eq!(overtime_nudge_milestone(45 * 60, 30), Some(45));
        assert_eq!(overtime_nudge_milestone(60 * 60, 45), Some(60));
        assert_eq!(overtime_nudge_milestone(74 * 60, 60), None);
        assert_eq!(overtime_nudge_milestone(75 * 60, 60), Some(75));
    }

    #[test]
    fn skips_rewind_scheduling_but_not_unbroken_work() {
        let mut engine = Engine {
            work: 20 * 60,
            unbroken_work: 45 * 60,
            brk: Some(Brk {
                long: false,
                dur: 25,
                t: 4,
            }),
            ..Engine::default()
        };

        finish_break(None, &mut engine, false);
        assert_eq!(engine.work, 15 * 60);
        assert_eq!(engine.unbroken_work, 45 * 60);
        assert_eq!(engine.returning, 1);

        engine.brk = Some(Brk {
            long: false,
            dur: 25,
            t: 25,
        });
        finish_break(None, &mut engine, true);
        assert_eq!(engine.work, 0);
        assert_eq!(engine.unbroken_work, 0);
        assert_eq!(engine.debt, 0);
        assert_eq!(engine.returning, 1);
    }

    #[test]
    fn afk_prebreak_and_typing_thresholds_are_exact() {
        assert!(!afk_should_arm(299, 300));
        assert!(afk_should_arm(300, 300));
        assert!(!afk_should_prompt(120, 120, true, false));
        assert!(afk_should_prompt(0, 120, true, false));
        assert!(!afk_should_prompt(0, 120, true, true));

        assert_eq!(countdown_remaining(100, 105), 5);
        assert_eq!(countdown_remaining(104, 105), 1);
        assert_eq!(countdown_remaining(105, 105), 0);
        assert_eq!(countdown_remaining(106, 105), 0);

        assert_eq!(due_action(true, false, 0, 100, false), DueAction::Wait);
        assert_eq!(
            due_action(true, false, 0, 100, true),
            DueAction::StartCountdown
        );
        assert_eq!(due_action(true, true, 105, 100, true), DueAction::Wait);
        assert_eq!(
            due_action(true, true, 105, 105, true),
            DueAction::StartBreak
        );
        assert_eq!(
            due_action(false, false, 0, 100, true),
            DueAction::StartBreak
        );

        assert!(!pending_break_ready(1_199, 1_200, 5, 100, 80, false));
        assert!(pending_break_ready(1_200, 1_200, 2, 100, 100, false));
        assert!(pending_break_ready(1_200, 1_200, 0, 120, 100, false));
        assert!(!pending_break_ready(1_200, 1_200, 0, 119, 100, false));
        assert!(pending_break_ready(1_200, 1_200, 0, 220, 100, true));
    }

    #[test]
    fn chill_and_locked_in_keep_distinct_skip_rules() {
        let mut engine = Engine {
            debt: 8,
            ..Engine::default()
        };

        engine.s.mode = "lenient".into();
        assert!(engine.skippable());
        assert_eq!(engine.retry_after_skip(), engine.interval());

        engine.s.mode = "focused".into();
        assert!(!engine.skippable());
        assert_eq!(engine.retry_after_skip(), 0);
    }
}
