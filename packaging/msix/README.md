# Microsoft Store MSIX

This package definition is associated with the GazeOff MSIX product in Partner Center.

## Store identity

- Identity name: `Zefyrus.GazeOff`
- Publisher: `CN=BF7FA1EC-A851-4489-839E-4802952A71DB`
- Publisher display name: `Zefyrus`
- Package family name: `Zefyrus.GazeOff_55fthz64p9jg2`
- Store ID: `9NRL1Z4MQTD8`
- Version: `1.0.4.0`
- Architecture: `x64`

## Build

Build a clean release executable before running a Tauri installer bundle, because installer bundling patches the executable's bundle metadata.

```powershell
cargo build --release
New-Item -ItemType Directory -Force target\msix-layout | Out-Null
Copy-Item target\release\gazeoff.exe target\msix-layout\GazeOff.exe

winapp cert generate `
  --manifest packaging\msix\Package.appxmanifest `
  --output target\msix-devcert.pfx `
  --if-exists Overwrite

winapp package target\msix-layout `
  --manifest packaging\msix\Package.appxmanifest `
  --cert target\msix-devcert.pfx `
  --cert-password password `
  --output target\release-assets\v0.1.4\GazeOff_1.0.4.0_x64.msix `
  --exe GazeOff.exe
```

Never commit the generated `.pfx` or `.cer` files. The self-signed development certificate is sufficient for Store submission because Microsoft re-signs an accepted MSIX package.
