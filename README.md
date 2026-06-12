# gazeOff

A quiet recovery companion for the Windows tray. Eyes, posture, breaks — without the nagging.

## Stack

- **Tauri v2 + Rust** — one source file ([src/main.rs](src/main.rs)) holds the whole engine.
- **Vanilla HTML/CSS/JS** — four static files in [ui/](ui/), no npm, no framework, no bundler.
- **Native Windows acrylic** (window-vibrancy) for real frost, Segoe UI Variable for type, tray icon drawn programmatically at runtime.
- Release binary: **~2.9 MB**, ~36 MB RAM idle.

## Build & run

```
cargo build --release
target\release\gazeoff.exe
```

The app lives entirely in the system tray (left-click → status panel, right-click → menu).
Settings persist to `%APPDATA%\com.gazeoff.app\gazeoff.json`.

## What's inside

- Short breaks (default 20 min / 40 s) with a pre-break heads-up pill, long breaks every 3rd cycle (5 min), long-session upgrade after 90 min.
- **Recovery-state model**: Clear → Accumulating → Due, driven by skip debt and session length. Tray icon, panel, overlay, and nudges all tint with the state (sage → amber → ember).
- A skipped break returns in 5 minutes, not 20 — escalation by patience, not punishment.
- Typing-aware delay (waits for a 2 s input pause), fullscreen smart pause with a configurable cooldown after it ends, configurable idle pause and absence reset.
- Blink + posture nudges with drawn glyphs and a dissolve timeline; calm + playful voice modes; three strictness modes; optional hold-firm after 3 skips.
- Active hours **and** active days (weekday chips), launch-with-Windows toggle, soft end-of-break chime (toggleable).
- Tray panel: live countdown ring, today's session timeline (sage = ended with a break, ember = skipped), breaks / skipped / longest / day-streak tiles.
- Evidence tooltips throughout settings; everything persists to one JSON file.
