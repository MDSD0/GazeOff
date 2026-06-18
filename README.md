<div align="center">
  <img src="website/assets/tray_blob.png" width="112" alt="GazeOff blob">
  <h1>GazeOff</h1>
  <p>A quiet Windows companion for healthier eyes, posture, and screen habits.</p>
  <p>
    <a href="https://github.com/MDSD0/GazeOff/releases/latest"><img alt="Latest release" src="https://img.shields.io/github/v/release/MDSD0/GazeOff?style=flat-square&color=6f86a8"></a>
    <a href="https://github.com/MDSD0/GazeOff/releases"><img alt="Downloads" src="https://img.shields.io/github/downloads/MDSD0/GazeOff/total?style=flat-square&color=6f86a8"></a>
    <a href="https://github.com/MDSD0/GazeOff/stargazers"><img alt="GitHub stars" src="https://img.shields.io/github/stars/MDSD0/GazeOff?style=flat-square&color=6f86a8"></a>
    <img alt="Windows 10 and 11" src="https://img.shields.io/badge/Windows-10%20%7C%2011-6f86a8?style=flat-square">
    <img alt="Built with Rust" src="https://img.shields.io/badge/built%20with-Rust-bb6b3d?style=flat-square">
  </p>
  <p>
    <a href="https://gazeoff.vercel.app"><strong>Website</strong></a>
    ·
    <a href="https://github.com/MDSD0/GazeOff/releases/latest"><strong>Download</strong></a>
    ·
    <a href="docs/FEATURE_CATALOG.md"><strong>Feature catalog</strong></a>
  </p>
</div>

---

GazeOff lives in the Windows tray and quietly watches the shape of a work session—not its content. It schedules short eye breaks, reminds you to blink and reset your posture, adapts around calls and fullscreen activity, and turns the day into a useful recovery score.

## Why it exists

Reading at a screen reduces spontaneous blinking, and electronic reading can increase incomplete blinks. Long, uninterrupted sessions also invite close focus and poor posture. GazeOff makes recovery brief, predictable, and difficult to forget without becoming another productivity dashboard demanding attention.

The recommended rhythm starts with:

- 20 minutes of work
- A 25-second distance break
- A 3-minute break every third cycle
- A cursor-following `5, 4, 3, 2, 1` warning before the screen changes

Every interval, sound, reminder, and visual can be adjusted.

## What it does

### Breaks that respect context

- Typing-aware break timing waits for a natural pause.
- Chill, Smart, and Locked In modes offer different levels of enforcement.
- Smart mode brings skipped breaks back progressively sooner.
- Extended sessions automatically graduate into a longer recovery break.
- Time-of-day skies, wallpaper blur, and a black-screen option shape the break experience.

### Knows when to stay quiet

- Holds breaks while the microphone or camera is active.
- Detects fullscreen video players, browser playback, and games.
- Pauses after configurable idle time.
- Asks whether time away from the computer counted as a real break.

### Small physical reminders

- Independent blink and posture timers.
- Animated, background-free reminder marks.
- Cursor, center, edge, and corner placement.
- Optional dimming, labels, sounds, and separate volume controls.

### Recovery you can see

- Live eye-recovery score.
- Screen time, completed breaks, skipped breaks, and time rested.
- Longest uninterrupted session and day streak.
- Seven-day history with details revealed on hover.
- Local session timeline distinguishing completed and skipped cycles.

## Local-first by design

GazeOff does not need an account or cloud dashboard. Settings, statistics, and daily history stay in a versioned JSON file on the computer:

```text
%APPDATA%\com.gazeoff.app\gazeoff.json
```

Windows activity signals are used only to decide whether a reminder should wait.

<details>
<summary><strong>What GazeOff detects—and what it never uploads</strong></summary>

- Keyboard and pointer idle time, to pause work-session counting.
- Foreground fullscreen state, to avoid interrupting films and games.
- Windows microphone and camera activity flags, to hold reminders during calls.
- Your current wallpaper only when wallpaper blur is selected.

These signals are processed on-device. GazeOff has no account system, analytics pipeline, or cloud sync.

</details>

## Install

Download the newest Windows build from [GitHub Releases](https://github.com/MDSD0/GazeOff/releases/latest), run `gazeoff.exe`, and find GazeOff in the system tray.

> The current release is not signed with a public code-signing certificate. Windows SmartScreen may show a warning on first launch.

## Development

Requirements:

- Rust stable toolchain
- Windows 10 or Windows 11
- WebView2 runtime

Run the development build:

```powershell
.\dev.ps1
```

Build the optimized executable:

```powershell
cargo build --release
.\target\release\gazeoff.exe
```

The interface is vanilla HTML, CSS, and JavaScript hosted inside Tauri v2. Native Windows APIs provide idle detection, foreground-app checks, microphone and camera activity, wallpaper access, autostart, and acrylic effects.

<details>
<summary><strong>Build and release notes</strong></summary>

The release artifact is built with Cargo's optimized release profile. Every GitHub Release includes a SHA-256 checksum so the downloaded executable can be verified with:

```powershell
Get-FileHash .\gazeoff.exe -Algorithm SHA256
```

Public code signing is not yet configured, so Windows SmartScreen can ask for confirmation on first launch.

</details>

## Repository map

```text
src/main.rs          Native scheduler, Windows integration, state and commands
ui/                  Tray panel, break overlay, Studio, reminders and prompts
website/             Public product website deployed on Vercel
icons/               Only the production app and tray icons
docs/                Feature catalog and publishing notes
capabilities/        Tauri permission configuration
```

## Links

- [GazeOff website](https://gazeoff.vercel.app)
- [Source code](https://github.com/MDSD0/GazeOff)
- [Latest release](https://github.com/MDSD0/GazeOff/releases/latest)
- [Complete feature catalog](docs/FEATURE_CATALOG.md)
- [Publishing checklist](docs/PUBLISHING_CHECKLIST.md)

<div align="center">
  <sub>Gaze off. See how beautiful the sky is.</sub>
</div>
