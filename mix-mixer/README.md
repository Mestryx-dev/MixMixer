# MixMixer

Minimal Windows tray app that mixes **voice (post-Equalizer APO)** with **soundboard audio** (WAV hotkeys + VB-Cable) and sends the result to a virtual microphone and optional headphone monitor.

## What it does

```
fifine Mic (post E-APO) ──► capture ──┐
CABLE Output (external sfx) ──► capture ──┼──► mix ──► CABLE Input (Discord micro)
WAV hotkeys (F1, F2, …) ────────────────┘              └──► fifine SC3 (monitor)
```

- **Discord / games / OBS** use `CABLE Output` as the microphone.
- **Equalizer APO** stays on the physical fifine microphone (unchanged).
- **Soundpad UniteFx** should be disabled to avoid double injection.

## Requirements

- Windows 10/11
- [VB-Audio Virtual Cable](https://vb-audio.com/Cable/)
- [Equalizer APO](https://sourceforge.net/projects/equalizerapo/) on the fifine mic
- [Rust toolchain](https://rustup.rs/) + MSVC Build Tools (for building from source)

## Build

```powershell
# From a Developer Command Prompt or after vcvars64.bat:
cd d:\Audio\mix-mixer
cargo build --release
```

Binary: `target\release\mix-mixer.exe`

## Setup

1. Copy `config.example.json` → `config.json` in the same folder as the exe.
2. Create a `sounds/` folder and add WAV files referenced in config bindings.
3. List devices to verify names:

   ```powershell
   mix-mixer --list-devices
   ```

4. **Windows audio**
   - Discord / games micro: **CABLE Output**
   - Equalizer APO: **fifine Microphone** (keep your VST chain)
5. **UniteFx** : laisser **inactif** si déjà le cas ; ne réactiver que si tu n'utilises plus MixMixer.
6. Run MixMixer from the folder containing `config.json`:

   ```powershell
   mix-mixer.exe
   ```

## Config

Device names are matched by **case-insensitive substring** (e.g. `"fifine Microphone"` matches `"Microphone (2- fifine Microphone)"`).

| Key | Default | Role |
|-----|---------|------|
| `devices.voice_input` | fifine Microphone | Post-E-APO voice capture |
| `devices.sfx_input` | CABLE Output | External soundboard via VB-Cable |
| `devices.virtual_mic_output` | CABLE Input | Virtual mic for Discord |
| `devices.monitor_output` | fifine SC3 | Headphone monitor |
| `gains.voice` / `gains.sfx` / `gains.master` | 1.0 / 0.85 / 1.0 | Mix levels |
| `monitor.enabled` | true | SC3 monitor on by default |
| `monitor.include_sfx` | true | Hear SFX in monitor |
| `buffer_frames` | 256 | cpal buffer (~5 ms at 48 kHz); increase to 512 if crackling |

See [`config.example.json`](config.example.json) for a full example.

## Tray menu

- **Réglages...** — fenêtre graphique (périphériques, écoute, niveaux)
- **Couper SFX** — voix seule vers le micro virtuel
- **Activer/désactiver écoute** — monitor casque à chaud
- **Recharger config** — relit `config.json` et redémarre l'audio
- **Quitter**

## Fenêtre Réglages

Clic droit sur l'icône tray → **Réglages...**

- Listes déroulantes pour micro, SFX, CABLE Input, monitor
- ☑ Activer l'écoute sur le casque
- ☑ Inclure les SFX dans l'écoute
- Sliders voix / SFX / master
- **Appliquer** — sauvegarde `config.json` et redémarre les flux audio

## Hotkeys

Configure in `config.json` under `soundboard.bindings`. Supported keys include `F1`–`F12`, `Numpad0`–`Numpad9`, and `Space`.

## External soundboard (VB-Cable)

To play audio from another app into the mix:

1. Set that app's output to **CABLE Input** (playback).
2. MixMixer captures **CABLE Output** and mixes it with voice + WAV hotkeys.

Do **not** route CABLE Output → CABLE Input without MixMixer in the chain (feedback loop).

## Troubleshooting

| Issue | Fix |
|-------|-----|
| Crackling / dropouts | Increase `buffer_frames` to 512 |
| Device not found | Run `--list-devices`, adjust substring in config |
| Discord silent | Confirm Discord micro = CABLE Output; MixMixer running |
| Double SFX / weird processing | Disable Soundpad UniteFx APO on mic |
| Sample rate mismatch | MixMixer resamples inputs automatically if native rate ≠ 48 kHz |

## Docs

- Technical spec: [`../docs/dev-mix-mixer.md`](../docs/dev-mix-mixer.md)
- Validation checklist: [`../docs/validate-mix-mixer.md`](../docs/validate-mix-mixer.md)
- Architecture decision: DEC-005 in [`../docs/decisions.md`](../docs/decisions.md)
