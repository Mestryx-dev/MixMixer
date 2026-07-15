# MixMixer v0.1.2

UI polish release: tray behavior, settings header language chips, footer GitHub link, and docs aligned with the current app.

## Highlights

- **Tray**: Double-click opens settings; right-click menu is **About** / **Quit** only
- **Settings**: EN / FR chips in the header under live metrics; routing toggle alone in GENERAL
- **UX**: Subtle row hover, toast on Apply, hide-to-tray on close/minimize
- **Footer**: `MixMixer vX.Y.Z` links to the GitHub repository
- **Docs**: Screenshot in README; public English docs match current UI

## Install (Windows x64)

1. Download **`MixMixer-v0.1.2-windows-x64.zip`** from [Assets](https://github.com/Mestryx-dev/MixMixer/releases/tag/v0.1.2).
2. Extract the folder.
3. Copy `config.example.json` → `config.json` beside `mix-mixer.exe`.
4. Run `mix-mixer.exe --list-devices` and edit device substrings in `config.json`.
5. Set Discord (and games/OBS) microphone to **CABLE Output**.
6. Launch `mix-mixer.exe`.

Full walkthrough: [docs/TUTORIAL.md](TUTORIAL.md)

## Upgrade from v0.1.1

- No breaking config changes.
- Tray menu items **Settings**, **Toggle monitor**, and **Reload config** were removed; use the settings window and double-click tray instead.
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
