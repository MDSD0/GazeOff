# Microsoft Store Listing — English (United States)

This copy is prepared for the first Microsoft Store submission of GazeOff v0.1.4.

## Product name

GazeOff

## Short description

Take better breaks without losing your flow. GazeOff brings quiet eye breaks, blink reminders, posture nudges, and recovery tools to Windows.

## Description

Take better breaks without losing your flow.

GazeOff is a quiet Windows tray companion for people who work, study, create, and play at a screen. It keeps eye breaks, blink reminders, posture nudges, and longer recovery sessions close without filling your desktop with another distracting app.

A calm timer lives in the system tray. When a break is due, GazeOff gives you a brief, customizable pause. Smart Pause can wait during fullscreen apps, games, video playback, calls, and idle time, so reminders arrive at a better moment.

Start with a balanced default rhythm:

• Work for 20 minutes
• Look into the distance for 25 seconds
• Take a 3-minute recovery break after four short breaks

Make it yours:

• Customize short and long break timing
• Turn blink and posture reminders on or off
• Choose calm break screens, sounds, and reminder positions
• Pause or start a break from the Windows system tray
• Review local recovery statistics and seven-day history

Your activity stays yours. GazeOff requires no account, has no advertising, and has no cloud dashboard. Settings, break history, and recovery statistics stay on your device. GazeOff does not record audio or video, capture your screen, or upload your activity.

GazeOff is a general wellness and productivity tool. It is not a medical device and does not provide medical advice.

Need help or have an idea? Visit https://github.com/MDSD0/GazeOff/issues or email mr.imcommon@gmail.com.

## What's new in this version

Leave this field **blank for the first submission**. Microsoft says this field is intended for updates to an existing Store app.

For the next update, use:

> Improved first-run guidance, local statistics, Smart Pause behavior, tray controls, blink and posture reminders, and calm break screens.

## Product features

Enter these as separate feature rows:

1. Take better breaks without losing your flow
2. Quiet blink and posture reminders
3. Smart Pause waits during calls, games, videos, fullscreen apps, and idle time
4. Calm break screens with adjustable timing, sounds, and visuals
5. Local recovery statistics and seven-day history
6. Fast controls from the Windows system tray
7. No account, advertising, or cloud dashboard

## Keywords

Partner Center permits at most 7 keyword terms, 40 characters per term, and 21 unique words across all terms. Enter these as seven separate terms:

1. eye break
2. blink reminder
3. posture reminder
4. screen time
5. digital wellbeing
6. work breaks
7. Windows tray app

## URLs

- Website URL: https://gazeoff.vercel.app/
- Support URL: https://github.com/MDSD0/GazeOff/issues
- Privacy policy URL: https://gazeoff.vercel.app/privacy

## Screenshots

Upload at least four clean Windows screenshots in this order:

1. **Welcome and tray guidance** — Show the first-run welcome screen and explain that GazeOff continues from the system tray.
2. **Recovery dashboard** — Show local recovery score, completed breaks, rest time, and history with believable fresh-user data.
3. **Break experience** — Show the calm break overlay with the GazeOff blob and remaining break time.
4. **Smart Pause and settings** — Show break timing plus fullscreen, call, video, and idle controls.
5. **Blink and posture reminders** — Show the rounded blink reminder or posture nudge in a realistic desktop context.

Do not include personal notifications, email addresses, file paths, debug windows, or the developer's own accumulated statistics. Capture at 1366 × 768 or larger, using PNG, with consistent Windows scaling.

The prepared v0.1.3 screenshot set and 1:1 Store logo are in `store-assets/microsoft-store-v0.1.3/images`. See the README in that folder for the exact Partner Center import paths.

## Store logo

Use the GazeOff blob—not an eye—as the Store identity. Start from `icons/tray_blob.png` and export the exact square dimensions requested by Partner Center. Keep generous padding around the blob and verify it remains recognizable at small sizes.

## Submission notes

- Select **English (United States)** and complete that listing.
- The description and at least one screenshot are required; Microsoft recommends four or more screenshots and permits up to ten.
- Use the same product name as the installed app: **GazeOff**.
- Upload `GazeOff_1.0.4.0_x64.msix` directly to the MSIX product in Partner Center.
- Store identity: `Zefyrus.GazeOff`, publisher `CN=BF7FA1EC-A851-4489-839E-4802952A71DB`, publisher display name `Zefyrus`.
- Package family name: `Zefyrus.GazeOff_55fthz64p9jg2`.
- Package version: `1.0.4.0`; architecture: `x64`; language: `en-us`.
- The development signature is intentionally self-signed. Microsoft Store replaces it with a Microsoft certificate after certification.
- Keep the NSIS EXE as the separate website download. Do not upload the unsigned EXE or MSI to the MSIX submission.

## Official Microsoft guidance

- [Use winapp CLI with Tauri](https://learn.microsoft.com/en-us/windows/apps/dev-tools/winapp-cli/guides/tauri)
- [MSIX package requirements](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/app-package-requirements)
- [Code-signing options](https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/code-signing-options)
