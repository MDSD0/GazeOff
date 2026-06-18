# Publishing Checklist

## Required Decisions

- Choose the public publisher name used by the website, installer, and future signing certificate.
- Choose and add a software license.
- Add the repository URL, support URL, and privacy contact.
- Decide whether releases remain self-signed, use SignPath for an eligible open-source project, or use a public-trust certificate.

## Application

- Run `cargo fmt -- --check`.
- Run `cargo clippy --all-targets --all-features -- -D warnings`.
- Run the unit test suite on a machine that permits the generated test executable.
- Smoke-test the release on a clean Windows user account.
- Verify launch-at-login, Smart Pause, AFK, sounds, wallpaper access, multi-monitor positioning, and DPI scaling.
- Produce a versioned installer or release archive.
- Sign the final executable and installer after all build steps.
- Publish SHA-256 checksums beside every downloadable artifact.

## Product And Website

- Paste and review the English Store copy in `MICROSOFT_STORE_LISTING.md`.
- Publish a privacy policy explaining local idle, foreground-app, microphone/camera activity, and wallpaper access.
- Capture final screenshots for Studio, sky mode, wallpaper mode, blink and posture reminders, AFK, and the tray panel.
- Use `FEATURE_CATALOG.md` as the website feature source of truth.
- Add support, issue-reporting, changelog, and download pages.
- Confirm website claims against the release build.

## Repository Hygiene

- Keep `target/` and `remove/` out of version control.
- Do not commit private keys, PFX or P12 files, local certificates, or account credentials.
- Keep raw design references and one-off migration scripts outside the publishable source tree.
- Tag releases and keep release notes aligned with the version in `tauri.conf.json`.
