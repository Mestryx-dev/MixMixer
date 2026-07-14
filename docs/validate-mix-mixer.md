# MixMixer — checklist de validation manuelle

> À exécuter sur la machine cible avec la chaîne E-APO fifine active.  
> Prérequis : VB-Cable installé. Soundpad UniteFx **inactif** (état local confirmé).

---

## Préparation

- [ ] `mix-mixer.exe` build release dans `mix-mixer/target/release/`
- [ ] `config.json` copié depuis `config.example.json`
- [ ] Dossier `sounds/` avec `applause.wav` et `bruh.wav` (ou fichiers configurés)
- [ ] Discord micro = **CABLE Output**
- [ ] Equalizer APO actif sur **fifine Microphone**

---

## Tests automatiques (déjà validés en dev)

| Test | Commande | Critère |
|------|----------|---------|
| Unit tests | `cargo test` | 3 tests OK (ring buffer, mixer, device match) |
| Liste devices | `mix-mixer --list-devices` | fifine Mic, CABLE Output/Input, SC3 visibles |

**Résultat dev (2026-07-14) :**

```
[in ] CABLE Output (VB-Audio Virtual Cable) — 48000 Hz, 2 ch, F32
[in ] Microphone (2- fifine Microphone) — 48000 Hz, 2 ch, F32
[out] Haut-parleurs (fifine SC3) — 48000 Hz, 2 ch, F32
[out] CABLE Input (VB-Audio Virtual Cable) — 48000 Hz, 2 ch, F32
```

---

## Tests manuels (utilisateur)

### 1 — Voix seule → micro virtuel

- [ ] Lancer `mix-mixer.exe`
- [ ] Parler dans le micro fifine
- [ ] Windows Paramètres → Son → Entrée **CABLE Output** → barre de niveau bouge
- [ ] Discord test vocal : voix audible, traitement E-APO présent (gate/EQ)

### 2 — Hotkey WAV

- [ ] Appuyer **F1** (ou binding configuré)
- [ ] SFX audible côté Discord
- [ ] Latence ressentie acceptable (< ~50 ms subjectif)

### 3 — SFX via VB-Cable externe

- [ ] Jouer un son depuis une app vers **CABLE Input** (playback)
- [ ] SFX mixé avec la voix dans Discord
- [ ] Simultané avec F1 : les deux sources audibles

### 4 — Monitor SC3

- [ ] Casque fifine SC3 entend voix + SFX (`monitor.include_sfx: true`)
- [ ] Tray → **Toggle monitor** : SC3 muet, Discord inchangé
- [ ] Remettre monitor ON

### 5 — Mute SFX (tray)

- [ ] Tray → **Toggle mute SFX**
- [ ] Discord : voix seule, pas de hotkey ni CABLE externe
- [ ] Désactiver mute : SFX reviennent

### 6 — Reload config

- [ ] Modifier `gains.sfx` dans config.json (ex. 0.5)
- [ ] Tray → **Reload config**
- [ ] Niveau SFX réduit sans redémarrage

### 7 — Sans Soundpad UniteFx (N/A si déjà inactif)

- [ ] UniteFx reste **inactif** sur le micro (Device Selector) — état actuel confirmé
- [ ] Pas de double traitement / pas d'écho fantôme avec MixMixer seul

---

## Critères de succès MVP

| Critère | Statut |
|---------|--------|
| Build release sans erreur | ✅ |
| `--list-devices` OK | ✅ |
| Voix → CABLE Output (Discord) | ⬜ manuel |
| Hotkey WAV → Discord | ⬜ manuel |
| VB-Cable sfx mixé | ⬜ manuel |
| Monitor SC3 | ⬜ manuel |
| Tray mute/reload | ⬜ manuel |

---

## Notes

- Si crackling : augmenter `buffer_frames` à 512 dans config.
- Changement de device : redémarrer MixMixer (reload ne rebind pas les streams).
- Logs : `RUST_LOG=mix_mixer=debug mix-mixer.exe`
