# MixMixer — spec technique

> App WASAPI Rust — mix voix post-E-APO + soundboard → micro virtuel VB-Cable.  
> Décision : **DEC-005** dans [decisions.md](decisions.md)

---

## Objectif

Un exe tray minimaliste qui remplace VoiceMeeter / Soundpad UniteFx pour :

1. Capturer `Microphone fifine` (signal post-Equalizer APO)
2. Mixer WAV internes (hotkeys) + `CABLE Output` (apps externes)
3. Sortir vers `CABLE Input` (Discord → `CABLE Output`)
4. Monitor sur `fifine SC3` (activé par défaut)

---

## Architecture

```
Voice capture ──► ring buffer ──┐
SFX capture   ──► ring buffer ──┼──► mix callback ──► CABLE Input
WAV player    ──────────────────┘              └──► monitor ring ──► SC3
```

- **Master clock** : callback de sortie `CABLE Input`
- **Ring buffers SPSC** entre captures et mix
- **Monitor** : ring buffer alimenté par le callback principal

---

## Stack

| Crate | Usage |
|-------|--------|
| cpal | WASAPI I/O |
| hound | WAV preload |
| rubato | Resampling fallback |
| tray-icon | System tray |
| global-hotkey | Hotkeys soundboard |

---

## Config

Voir [`mix-mixer/config.example.json`](../mix-mixer/config.example.json).

Devices matchés par **substring** insensible à la casse.

---

## Setup Windows

1. Equalizer APO reste sur **fifine Microphone**
2. Désactiver Soundpad UniteFx APO (Device Selector)
3. Discord micro = **CABLE Output**
4. Lancer `mix-mixer.exe` depuis le dossier contenant `config.json`

---

## CLI

```bash
mix-mixer --config config.json
mix-mixer --list-devices
```

---

## Validation

Checklist manuelle : [`validate-mix-mixer.md`](validate-mix-mixer.md)
