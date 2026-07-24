# MixMixer v0.1.4

Stable release: live settings, listen volume only, multi-monitor size fixes.

## Highlights

- **Live settings**: every change takes effect immediately — no **Apply** / **Cancel**. Config auto-saves to `%APPDATA%\MixMixer\config.json`.
- **Listen volume** only: headphone monitor level (`monitor.volume`). No Voice/Master sliders; Discord/CABLE stays at unity. Volume updates while you drag the slider.
- **Multi-monitor / DPI**: maximize disabled, no oversized restore, fixed size re-applied when scale factor or size drifts.

## Install (Windows x64)

1. Download **`MixMixer-v0.1.4-windows-x64.zip`** from [Assets](https://github.com/Mestryx-dev/MixMixer/releases/tag/v0.1.4).
2. Install [VB-Audio Virtual Cable](https://vb-audio.com/Cable/) if needed.
3. Extract and double-click `mix-mixer.exe`.
4. Enable **Headphone monitor** and use **Listen volume** as needed.
5. Discord / games / OBS mic = **CABLE Output**.

Full walkthrough: [docs/TUTORIAL.md](https://github.com/Mestryx-dev/MixMixer/blob/main/docs/TUTORIAL.md)

## Upgrade notes

- Apply / Cancel are gone; changes apply live.
- Old `gains.voice` / `gains.master` keys are ignored; the next auto-save rewrites config without them.
- Add `"volume": 1.0` under `monitor` if missing (created automatically for new installs).

## Package contents

```
MixMixer-v0.1.4-windows-x64/
  mix-mixer.exe
  config.example.json
  LICENSE
```

## Full changelog

[CHANGELOG.md](https://github.com/Mestryx-dev/MixMixer/blob/main/CHANGELOG.md)
