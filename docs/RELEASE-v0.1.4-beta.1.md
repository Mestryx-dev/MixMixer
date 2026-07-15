# MixMixer v0.1.4-beta.1

**Pre-release / beta** — please report multi-monitor sizing issues if anything remains odd.

## Highlights

- **Listen volume** only: headphone monitor level (`monitor.volume`). No Voice/Master sliders; Discord/CABLE stays at unity.
- **Multi-monitor / DPI**: disable maximize, stop restoring oversized windows, re-apply fixed size when scale factor or size drifts.
- Config still lives in `%APPDATA%\MixMixer\config.json` (auto-created on first run).

## Install (Windows x64)

1. Download **`MixMixer-v0.1.4-beta.1-windows-x64.zip`** from [Assets](https://github.com/Mestryx-dev/MixMixer/releases/tag/v0.1.4-beta.1).
2. Extract and double-click `mix-mixer.exe` (VB-Cable required).
3. Enable **Headphone monitor** and use **Listen volume** to turn yourself down in headphones.
4. Discord / games mic = **CABLE Output**.

Walkthrough: [docs/TUTORIAL.md](https://github.com/Mestryx-dev/MixMixer/blob/main/docs/TUTORIAL.md)

## Upgrade notes

- Old `gains.voice` / `gains.master` keys are ignored; next Apply rewrites config without them.
- Add `"volume": 1.0` under `monitor` if missing (created automatically for new installs).

## Feedback

Open a [bug report](https://github.com/Mestryx-dev/MixMixer/issues/new?template=bug_report.yml) with Windows version, monitor layout (resolutions + scaling %), and steps.

## Full changelog

[CHANGELOG.md](https://github.com/Mestryx-dev/MixMixer/blob/main/CHANGELOG.md)
