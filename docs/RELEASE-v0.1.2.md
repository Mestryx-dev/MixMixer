# MixMixer v0.1.2

UI polish release: tray behavior, settings header language chips, footer GitHub link, and docs aligned with the current app.

## Highlights

- **Tray**: Left-click or double-click opens settings; right-click menu is **About** / **Quit** only
- **Settings**: EN / FR chips in the header under live metrics; routing toggle alone in GENERAL
- **UX**: Subtle row hover, toast on Apply, hide-to-tray on close/minimize; taskbar restores when minimized
- **Footer**: `MixMixer vX.Y.Z` links to the GitHub repository
- **Docs**: Screenshot in README; public English docs match current UI

## Install (Windows x64)

1. Download **`MixMixer-v0.1.2-windows-x64.zip`** from [Assets](https://github.com/Mestryx-dev/MixMixer/releases/tag/v0.1.2).
2. Extract and run `mix-mixer.exe` (VB-Cable must be installed).
3. A default config is created at `%APPDATA%\MixMixer\config.json` — check devices in Settings.
4. Set Discord (and games/OBS) microphone to **CABLE Output**.

Full walkthrough: [docs/TUTORIAL.md](https://github.com/Mestryx-dev/MixMixer/blob/main/docs/TUTORIAL.md)

> **Note:** Newer builds auto-create AppData config. Older zip contents may still ship `config.example.json` as a reference only.

## Upgrade from v0.1.1

- No breaking config changes.
- Tray menu items **Settings**, **Toggle monitor**, and **Reload config** were removed; use the settings window and tray left-click / double-click instead.
- Language can still use `"locale"` / `MIXMIXER_LANG`; the header FR / EN chips also write `locale` to config.

## Package contents

```
MixMixer-v0.1.2-windows-x64/
  mix-mixer.exe
  config.example.json
  LICENSE
```

## Full changelog

[CHANGELOG.md](../CHANGELOG.md)
