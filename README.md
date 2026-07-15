# MixMixer

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%2F11-lightgrey)](https://github.com/Mestryx-dev/MixMixer)

**MixMixer** is a lightweight Windows system-tray application that routes your **post–Equalizer APO microphone** to **VB-Audio Virtual Cable** with minimal latency. Discord, games, and OBS use **CABLE Output** as the microphone input while your physical mic stays on the E-APO chain.

## Signal flow

```
Physical mic (post E-APO) ──► MixMixer ──► CABLE Input ──► CABLE Output ──► Discord / games / OBS
                                  │
                                  └──► Headphones (optional monitor)

External soundboard / browser ──► CABLE Input (separate app; Windows mixes with voice)
```

- MixMixer forwards **voice only** to **CABLE Input**.
- Set Discord and games to **CABLE Output** — not the physical mic.
- Soundboard apps can send audio to **CABLE Input** independently; Windows mixes both streams.

## Features

- Low-latency WASAPI capture and playback (Rust + cpal)
- iOS-style settings window (egui) with live latency metrics
- System tray controls: settings, monitor toggle, reload config, quit
- Auto-reconnect when Windows audio devices change (e.g. Discord switching devices)
- Optional headphone monitor bus
- English and French UI (`locale` in config or `MIXMIXER_LANG`)

## Requirements

| Component | Notes |
|-----------|--------|
| Windows 10/11 | x64 |
| [VB-Audio Virtual Cable](https://vb-audio.com/Cable/) | Required — install as administrator |
| [Equalizer APO](https://sourceforge.net/projects/equalizerapo/) | Optional — VST chain on physical mic |
| [Rust toolchain](https://rustup.rs/) + MSVC | For building from source |

## Quick start

### 1. Build

```powershell
cd mix-mixer
cargo build --release
```

Binary: `mix-mixer\target\release\mix-mixer.exe`

### 2. Configure

```powershell
copy config.example.json config.json
.\target\release\mix-mixer.exe --list-devices
```

Edit `config.json` — device names use **case-insensitive substring** matching.

### 3. Wire your apps

1. **Equalizer APO** → physical microphone (e.g. fifine).
2. **MixMixer** → captures that mic, outputs to **CABLE Input**.
3. **Discord / GTA / OBS** → microphone = **CABLE Output**.

### 4. Run

Place `mix-mixer.exe` next to `config.json` and launch. The settings window opens on startup; double-click the tray icon to reopen it.

Full walkthrough: [docs/TUTORIAL.md](docs/TUTORIAL.md)

## Configuration

| Key | Default | Description |
|-----|---------|-------------|
| `locale` | `en` | UI language: `en` or `fr` (overridden by `MIXMIXER_LANG`) |
| `devices.voice_input` | — | Capture device (post-E-APO mic) |
| `devices.virtual_mic_output` | — | VB-Cable input endpoint |
| `devices.monitor_output` | — | Headphone monitor output |
| `gains.voice` / `gains.master` | `1.0` | Level multipliers |
| `monitor.enabled` | `false` | Headphone monitor on/off |
| `buffer_frames` | `128` | ~2.7 ms @ 48 kHz; increase if crackling |
| `enabled` | `true` | Start with routing active |
| `sample_rate` | `48000` | Sample rate (Hz) |

See [`mix-mixer/config.example.json`](mix-mixer/config.example.json).

### Language

Set in `config.json`:

```json
"locale": "fr"
```

Or override at runtime:

```powershell
$env:MIXMIXER_LANG = "fr"
.\mix-mixer.exe
```

All UI strings live in [`mix-mixer/src/i18n/`](mix-mixer/src/i18n/) — add a new locale file and match arm in `Locale` to extend.

## Settings window

| Control | Action |
|---------|--------|
| Device pickers | Microphone, virtual mic, monitor output |
| Headphone monitor | Toggle local monitoring |
| Sliders | Voice gain, master, buffer size |
| **Apply** | Save and activate (window stays open) |
| **Cancel** | Revert to last applied values |
| **Quit** | Exit MixMixer |

Live metrics in the header: routing state, estimated latency, buffer fill.

## Troubleshooting

| Issue | Fix |
|-------|-----|
| Crackling / dropouts | Raise `buffer_frames` to 256 or 512 |
| Device not found | Run `--list-devices`, adjust substring in config |
| Discord silent | Mic = **CABLE Output**; routing enabled in MixMixer |
| Silence after Discord device change | Wait for auto-reconnect; do not pick physical mic in Discord |
| Debug logs | `$env:RUST_LOG='mix_mixer=info'; .\mix-mixer.exe` |

## Development

```powershell
cd mix-mixer
cargo build
cargo run -- --list-devices
```

Internal docs (French): [`docs/dev-mix-mixer.md`](docs/dev-mix-mixer.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Bug reports and feature requests: [GitHub Issues](https://github.com/Mestryx-dev/MixMixer/issues).

## License

MIT — see [LICENSE](LICENSE).

## Changelog

See [CHANGELOG.md](CHANGELOG.md).
