# MixMixer v0.1.3

Zero-config release: AppData settings with smart first-run defaults.

## Highlights

- **AppData config**: `%APPDATA%\MixMixer\config.json` created automatically on first launch
- **Smart defaults**: Windows default microphone, **CABLE Input** when present, system language when `en`/`fr`
- **Double-click ready**: no need to copy `config.example.json` next to the exe
- **About** dialog shows the active config path; `--print-config-path` for CLI

## Install (Windows x64)

1. Download **`MixMixer-v0.1.3-windows-x64.zip`** from [Assets](https://github.com/Mestryx-dev/MixMixer/releases/tag/v0.1.3).
2. Install [VB-Audio Virtual Cable](https://vb-audio.com/Cable/) if needed.
3. Extract and double-click `mix-mixer.exe`.
4. Confirm devices in Settings; set Discord / games / OBS mic to **CABLE Output**.

Reset config anytime by deleting `%APPDATA%\MixMixer\config.json` and relaunching.

Full walkthrough: [docs/TUTORIAL.md](TUTORIAL.md)

## Upgrade from v0.1.2

- Existing `config.json` next to an old exe is **not** used by default anymore.
- First run of v0.1.3 creates AppData config. To keep old settings, copy them to `%APPDATA%\MixMixer\config.json`, or pass `--config` once.
- `config.example.json` in the zip is a schema reference only.

## Package contents

```
MixMixer-v0.1.3-windows-x64/
  mix-mixer.exe
  config.example.json
  LICENSE
```

## Full changelog

[CHANGELOG.md](../CHANGELOG.md)
