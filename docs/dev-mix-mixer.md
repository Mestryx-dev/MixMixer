# MixMixer — spec technique v0.1.2

> App WASAPI Rust — micro post-E-APO → VB-Cable, latence minimale.  
> Décisions : **DEC-005**, **DEC-006** dans [decisions.md](decisions.md)

---

## Objectif

Un exe tray minimaliste qui :

1. Capture `Microphone fifine` (signal post-Equalizer APO)
2. Écrit vers `CABLE Input` (Discord / jeux lisent `CABLE Output`)
3. Monitor optionnel sur `fifine SC3`
4. Reconnexion automatique si Windows coupe les flux (Discord, GTA, etc.)

**Hors scope v0.1 :** soundboard interne (WAV hotkeys), capture CABLE Output, mix sfx dans l'app.

Les apps externes (soundboard, navigateur) envoient vers **CABLE Input** séparément ; Windows mixe au niveau OS.

---

## Architecture

```
Voice capture (fifine) ──► ring buffer ──► output callback ──► CABLE Input
                                              └──► monitor ring ──► SC3 (si activé)
```

- **Master clock** : callback de sortie `CABLE Input`
- **Ring buffer SPSC** entre capture micro et rendu
- **Monitor** : ring buffer alimenté par le callback principal
- **Métriques** : `AudioMetrics` partagé (atomics) entre thread audio et UI

---

## Stack

| Crate | Usage |
|-------|--------|
| cpal | WASAPI I/O (mode shared) |
| rubato | Resampling fallback si device ≠ 48 kHz |
| tray-icon | Icône barre des tâches |
| eframe / egui | Fenêtre Réglages |
| crossbeam-channel | Events app ↔ audio |
| tracing | Logs (`RUST_LOG=mix_mixer=info`) |

**Retiré depuis MVP initial :** `hound`, `global-hotkey`, module soundboard.

---

## Config

Voir [`mix-mixer/config.example.json`](../mix-mixer/config.example.json).

| Champ | Description |
|-------|-------------|
| `enabled` | Routage actif (persisté, toggle UI) |
| `buffer_frames` | 128 / 256 / 512 — redémarrage flux si changé |
| `devices.*` | Substring insensible à la casse |
| `monitor.enabled` | Stream monitor SC3 (redémarrage si changé) |
| `gains.*` | Reload à chaud via Appliquer |

---

## UI

### Fenêtre Réglages

- Titre fenêtre : « MixMixer — Réglages » / « MixMixer — Settings »
- Header : métriques live + puces FR / EN
- Sections : GENERAL (routage), DEVICES, AUDIO (monitor, gains, buffer)
- Footer : Appliquer / Annuler + version cliquable (GitHub)
- Toast court après Appliquer
- Fermer (×) ou minimiser → **hide to tray** (pas de quit)

### Tray

| Action | Effet |
|--------|--------|
| Clic gauche **ou** double-clic | Rouvrir les réglages |
| Clic droit → À propos / About | Dialog native Windows |
| Clic droit → Quitter / Quit | Arrêt de l'app |
| Clic barre des tâches (fenêtre minimisée) | Restaurer les réglages |

Menu tray volontairement minimal (plus d'« Écoute », « Recharger », « Réglages » dans le menu).

### Métriques

| Métrique | Calcul |
|----------|--------|
| Délai estimé | `2 × buffer_frames / sample_rate × 1000` ms |
| Buffer % | `voice_rb.available() / capacity` |
| État | actif / reconnexion / off |

---

## Threading

```
Main thread     : boucle AppEvent (audio cmds, About MessageBox, Quit)
Audio thread    : AudioEngine (cpal streams, reconnect)
Settings thread : eframe/winit + TrayHandle (any_thread Windows)
```

### Tray sur Windows (critique)

`tray-icon` exige un **event loop Win32** sur le thread qui possède l'icône. Chez MixMixer :

1. `TrayHandle` est créé dans le callback `eframe::run_native` (même thread winit).
2. `TrayIconEvent::set_event_handler` restaure la fenêtre tout de suite (`Visible` / `Minimized(false)` / `Focus`) — la boucle egui peut être inactive quand la fenêtre est minimisée.
3. `MenuEvent` (About / Quit) est pollé dans `SettingsApp::update` et renvoyé vers `AppEvent`.
4. La minimisation utilise `ViewportCommand::Minimized(true)` ; la restauration barre des tâches détecte la transition `minimized true → false`.

**Ne pas** créer le tray sur le thread main (poll + sleep) : les événements tray/menu n'arrivent jamais.

---

## Reconnexion audio

Si un stream cpal signale une erreur (`device no longer available`) :

1. Pause + libération de tous les flux
2. Retry avec backoff (400 ms → 5 s max)
3. Métrique « Reconnexion… » dans l'UI

Typique quand Discord ou GTA change le périphérique micro.

---

## Setup Windows

1. Installer [VB-Audio Virtual Cable](https://vb-audio.com/Cable/)
2. Equalizer APO sur **fifine Microphone**
3. Discord / GTA micro = **CABLE Output**
4. Lancer `mix-mixer.exe` (release = sans terminal)

---

## CLI

```powershell
mix-mixer --config config.json
mix-mixer --list-devices
```

---

## Build release

- `windows_subsystem = "windows"` — pas de console au double-clic
- Logs : lancer depuis PowerShell avec `RUST_LOG=mix_mixer=info`

---

## Git

Dépôt local : `d:\Audio\`  
Commit baseline v0.1 : `8c50c7d`

---

## Évolutions possibles

- Soundboard intégrée (WAV hotkeys) — DEC-005 initial, retiré DEC-006
- Capture CABLE Output pour mix sfx dans l'app
- Second câble VB (A+B) pour éviter conflits soundboard / voix
- Métriques latence mesurée (pas seulement estimée)
- Lancement au démarrage Windows

---

## Validation

Checklist : [`validate-mix-mixer.md`](validate-mix-mixer.md)
