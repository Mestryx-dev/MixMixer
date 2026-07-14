# MixMixer

Application Windows tray minimaliste : **micro post-Equalizer APO → VB-Cable** avec latence minimale.

## Ce que ça fait

```
fifine Mic (post E-APO) ──► MixMixer ──► CABLE Input ──► CABLE Output ──► Discord / GTA / OBS
                              │
                              └──► fifine SC3 (monitor, optionnel)

Apps / soundboard / navigateur ──► CABLE Input (séparément, mix Windows)
```

- **MixMixer** route uniquement le micro vers **CABLE Input**.
- **Discord / jeux** : micro = **CABLE Output** (pas le fifine directement).
- **Soundboard externe** : envoyer l'audio vers **CABLE Input** depuis une autre app ; Windows mixe avec le micro.

## Prérequis

- Windows 10/11
- **[VB-Audio Virtual Cable](https://vb-audio.com/Cable/)** (obligatoire) — installer en administrateur, redémarrer si demandé
- [Equalizer APO](https://sourceforge.net/projects/equalizerapo/) sur le micro fifine (optionnel, pour la chaîne VST)
- [Rust toolchain](https://rustup.rs/) + MSVC Build Tools (pour compiler)

## Build

```powershell
cd d:\Audio\mix-mixer
cargo build --release
```

Binaire : `target\release\mix-mixer.exe`

## Installation

1. Copier `config.example.json` → `config.json` (même dossier que l'exe).
2. Lister les périphériques :

   ```powershell
   mix-mixer --list-devices
   ```

3. Ajuster les noms dans `config.json` (substring insensible à la casse).
4. **Discord / GTA** : micro = **CABLE Output**.
5. **Equalizer APO** : reste sur **fifine Microphone**.
6. Lancer MixMixer (double-clic ou depuis le dossier contenant `config.json`).

## Config

| Clé | Défaut | Rôle |
|-----|--------|------|
| `devices.voice_input` | fifine Microphone | Capture micro post-E-APO |
| `devices.virtual_mic_output` | CABLE Input | Sortie vers micro virtuel |
| `devices.monitor_output` | fifine SC3 | Écoute casque (optionnel) |
| `gains.voice` / `gains.master` | 1.0 / 1.0 | Niveaux |
| `monitor.enabled` | false | Monitor casque |
| `buffer_frames` | 128 | Latence (~5 ms à 48 kHz) ; 512 si crackling |
| `enabled` | true | Routage actif au démarrage |
| `sample_rate` | 48000 | Taux d'échantillonnage |

Voir [`config.example.json`](config.example.json).

## Fenêtre Réglages

S'ouvre au démarrage. Clic tray ou double-clic icône → **Réglages...**

| Contrôle | Action |
|----------|--------|
| Listes déroulantes | Micro, CABLE Input, monitor casque |
| ☑ Écoute casque | Monitor SC3 |
| Sliders | Gain voix, master, buffer |
| **Appliquer** | Enregistre et active (fenêtre reste ouverte) |
| **Annuler** | Restaure les valeurs du dernier Appliquer |
| **Activer / Désactiver** | Coupe ou reprend le routage micro → VB-Cable |
| **Quitter** | Ferme MixMixer |

**Métriques** (coin bas droit, temps réel) :

- **Délai ~ X ms** — latence estimée (buffers capture + sortie)
- **Buffer X%** — remplissage tampon voix
- **Audio actif · N flux** — état du routage

## Menu tray

- **Réglages...**
- **Activer/désactiver écoute** — monitor casque à chaud
- **Recharger config** — relit `config.json` et redémarre l'audio
- **Quitter**

## Dépannage

| Problème | Solution |
|----------|----------|
| Crackling | Augmenter `buffer_frames` à 256 ou 512 |
| Périphérique introuvable | `mix-mixer --list-devices`, ajuster substring |
| Discord muet | Micro Discord = **CABLE Output** ; routage MixMixer activé |
| Crash / silence après changement micro Discord/GTA | Attendre reconnexion auto ; ne pas sélectionner fifine dans Discord |
| Voir les logs | `$env:RUST_LOG='mix_mixer=info'; .\mix-mixer.exe` (ouvre un terminal) |

## Docs

- Spec : [`../docs/dev-mix-mixer.md`](../docs/dev-mix-mixer.md)
- Validation : [`../docs/validate-mix-mixer.md`](../docs/validate-mix-mixer.md)
- Décisions : DEC-005 / DEC-006 dans [`../docs/decisions.md`](../docs/decisions.md)
