# MixMixer (crate)

Windows tray app: post-E-APO microphone → VB-Cable with minimal latency.

**Documentation:** see the [repository README](../README.md).

- [Setup tutorial](../docs/TUTORIAL.md)
- [Example config schema](config.example.json) (reference only)
- [UI strings / i18n](src/i18n/)

```powershell
cargo build --release
.\target\release\mix-mixer.exe
.\target\release\mix-mixer.exe --print-config-path
.\target\release\mix-mixer.exe --list-devices
```

User config is created at `%APPDATA%\MixMixer\config.json` on first run.
