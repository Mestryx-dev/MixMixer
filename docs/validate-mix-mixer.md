# MixMixer — checklist de validation manuelle

> v0.1 — routage micro → VB-Cable uniquement.  
> Prérequis : [VB-Audio Virtual Cable](https://vb-audio.com/Cable/) installé. Soundpad UniteFx **inactif**.

---

## Préparation

- [ ] `mix-mixer.exe` build release : `mix-mixer/target/release/`
- [ ] `config.json` copié depuis `config.example.json`
- [ ] Discord / GTA micro = **CABLE Output** (pas fifine)
- [ ] Equalizer APO actif sur **fifine Microphone**
- [ ] MixMixer routage **activé** (bouton ou `enabled: true`)

---

## Tests automatiques (dev)

| Test | Commande | Critère |
|------|----------|---------|
| Unit tests | `cargo test` | ring buffer, mixer, device match OK |
| Liste devices | `mix-mixer --list-devices` | fifine, CABLE Input/Output, SC3 visibles |

**Résultat dev (2026-07-14) :**

```
[in ] Microphone (2- fifine Microphone) — 48000 Hz, 2 ch, F32
[in ] CABLE Output (VB-Audio Virtual Cable) — 48000 Hz, 2 ch, F32
[out] CABLE Input (VB-Audio Virtual Cable) — 48000 Hz, 2 ch, F32
[out] Haut-parleurs (fifine SC3) — 48000 Hz, 2 ch, F32
```

---

## Tests manuels

### 1 — Voix → micro virtuel

- [ ] Lancer MixMixer (pas de terminal en release)
- [ ] Fenêtre Réglages s'ouvre ; métriques bas droite : **Audio actif**, délai ~5 ms (buffer 128)
- [ ] Parler dans le micro fifine
- [ ] Windows → Son → Entrée **CABLE Output** : barre de niveau bouge
- [ ] Discord test vocal : voix audible, traitement E-APO présent

### 2 — Activer / Désactiver

- [ ] Cliquer **Désactiver** : métriques → **Audio off**, Discord ne reçoit plus la voix
- [ ] Cliquer **Activer** : routage reprend, voix de retour dans Discord

### 3 — Appliquer / Annuler

- [ ] Changer gain voix, **Appliquer** : niveau change dans Discord
- [ ] Changer gain sans appliquer, **Annuler** : valeur précédente restaurée

### 4 — Monitor SC3 (optionnel)

- [ ] Cocher « Écoute casque », **Appliquer**
- [ ] Entendre sa voix dans le casque SC3
- [ ] Tray → **Activer/désactiver écoute** : SC3 muet, Discord inchangé

### 5 — Reconnexion (Discord / GTA)

- [ ] Routage actif, parler dans Discord
- [ ] Changer le micro dans Discord (CABLE Output ↔ autre) puis revenir sur CABLE Output
- [ ] MixMixer affiche **Reconnexion…** puis **Audio actif** ; voix revient sans redémarrer l'app

### 6 — Soundboard externe (mix OS)

- [ ] Jouer un son depuis une app vers **CABLE Input** (playback)
- [ ] Discord entend voix MixMixer + son externe mixés par Windows

### 7 — Buffer / latence

- [ ] Buffer 128 : délai affiché ~5 ms
- [ ] Buffer 512 : pas de crackling si crackling avant

---

## Critères de succès v0.1

| Critère | Statut |
|---------|--------|
| Build release sans erreur | ✅ |
| `--list-devices` OK | ✅ |
| Pas de terminal au démarrage release | ✅ |
| Voix → CABLE Output (Discord) | ⬜ manuel |
| Activer / Désactiver routage | ⬜ manuel |
| Reconnexion auto Discord/GTA | ⬜ manuel |
| Métriques temps réel UI | ✅ |
| Monitor SC3 | ⬜ manuel |

---

## Notes

- Crackling : augmenter `buffer_frames` à 256 ou 512.
- Logs debug : `$env:RUST_LOG='mix_mixer=debug'; .\mix-mixer.exe`
- Ne **pas** sélectionner le fifine dans Discord — conflit avec MixMixer.
