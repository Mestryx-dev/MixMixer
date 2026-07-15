# MixMixer (crate)

Windows tray app: post-E-APO microphone → VB-Cable with minimal latency.

**Documentation:** see the [repository README](../README.md).

- [Setup tutorial](../docs/TUTORIAL.md)
- [Example config](config.example.json)
- [UI strings / i18n](src/i18n/)

```powershell
cargo build --release
copy config.example.json config.json
.\target\release\mix-mixer.exe --list-devices
```
