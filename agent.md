# Agent Audio — Routage micro MixMixer

> Hub agent — contexte riche et statut courant.  
> **Documentation canonique :** [`docs/index.md`](docs/index.md)  
> Dernière mise à jour : **2026-07-14**

---

## Objectifs

1. **Envoyer le micro fifine post-E-APO** vers le micro virtuel **VB-Cable** pour Discord, jeux, OBS.
2. **Latence minimale** — buffer 128 frames par défaut (~5 ms estimé).
3. **Monitor casque optionnel** (fifine SC3) sans passer par VoiceMeeter.

---

## État actuel du système

| Élément | État |
|---------|------|
| **Micro actif** | fifine Microphone — E-APO (8 VST) |
| **Casque / sortie** | fifine SC3 — sans Equalizer APO |
| **Routage micro** | **MixMixer v0.1** — fifine → CABLE Input |
| **Soundboard externe** | Apps / navigateur → **CABLE Input** (mix Windows, séparé de MixMixer) |
| **Soundpad** | Installé — **non lancé, UniteFx inactif** |
| **Câble virtuel** | [VB-Audio Virtual Cable](https://vb-audio.com/Cable/) — **obligatoire** |
| **Mixeur** | **MixMixer** (remplace VoiceMeeter pour le routage micro) |
| **Interface Behringer** | Pilote OK — **déconnectée** |
| **Git local** | Dépôt `d:\Audio\` — commit baseline `8c50c7d` |

### Schéma actuel (MixMixer v0.1)

```
Micro fifine → E-APO (8 VST) → MixMixer capture
                                    │
                                    ├──► CABLE Input → CABLE Output → Discord / GTA / OBS
                                    └──► fifine SC3 (monitor, optionnel)

Apps / soundboard / navigateur → CABLE Input (séparément, mix OS)
```

**Important :** Discord et GTA doivent utiliser **CABLE Output** comme micro, **pas** le fifine directement.

Référence devices : [`docs/devices.md`](docs/devices.md)  
Chaîne VST : [`docs/equalizer-apo.md`](docs/equalizer-apo.md)  
Architecture : [`docs/architecture.md`](docs/architecture.md)  
Spec dev : [`docs/dev-mix-mixer.md`](docs/dev-mix-mixer.md)  
Validation : [`docs/validate-mix-mixer.md`](docs/validate-mix-mixer.md)  
Code : [`mix-mixer/`](mix-mixer/)

---

## MixMixer — fonctionnalités v0.1

| Fonction | Détail |
|----------|--------|
| **Fenêtre Réglages** | Périphériques, gains, buffer, monitor, Appliquer / Annuler / Quitter |
| **Activer / Désactiver** | Coupe ou reprend le routage micro → VB-Cable sans quitter l'app |
| **Métriques temps réel** | Coin bas droit : délai estimé, buffer %, état audio, nb flux |
| **Tray** | Réglages, écoute casque, recharger config, quitter |
| **Reconnexion auto** | Si Discord/GTA perturbe le micro, reconnexion automatique |
| **Release sans terminal** | Pas de fenêtre console au démarrage (double-clic exe) |

---

## Contexte technique

### Le problème Windows

Windows expose **un flux d'entrée** par micro. Pour que Discord entende le micro traité par E-APO **via** VB-Cable :

- MixMixer capture le fifine (post-E-APO) et écrit vers **CABLE Input**.
- Discord lit **CABLE Output** comme micro virtuel.

Equalizer APO **traite** le flux micro ; MixMixer **route** ce flux vers VB-Cable.

### Ce qui est en place

| Composant | Rôle |
|-----------|------|
| **Equalizer APO 1.4.2** | Chaîne voix (gate, EQ, saturation, bass, limiteurs) sur fifine |
| **MixMixer** | Routage micro → CABLE Input + monitor SC3 optionnel |
| **VB-CABLE** | Pont CABLE Input ↔ CABLE Output |
| **Soundpad Demo** | Installé — inactif ; soundboard via apps → CABLE Input si besoin |

### Risque latence

Chaîne VST lourde (8 plugins, 2× Limiter6 avec latence). Premier levier si retard audible : alléger E-APO.

---

## Décisions

| ID | Résumé | Statut |
|----|--------|--------|
| **DEC-005** | MixMixer app Rust WASAPI | accepté |
| **DEC-006** | Simplification v0.1 : routage voix seul, pas de soundboard interne | accepté |

Détail : [`docs/decisions.md`](docs/decisions.md)

---

## Prochaines étapes

1. [ ] **Validation manuelle Discord** — [`docs/validate-mix-mixer.md`](docs/validate-mix-mixer.md)
2. [ ] **Tester reconnexion** — changer micro dans Discord/GTA, vérifier retour auto
3. [ ] **Améliorations futures** — voir section « Évolutions » dans `docs/dev-mix-mixer.md`

---

## Documentation canonique

| Document | Rôle |
|----------|------|
| [`docs/index.md`](docs/index.md) | Index et conventions |
| [`docs/audit/2026-07-14.md`](docs/audit/2026-07-14.md) | Snapshot audit initial (immutable) |
| [`docs/devices.md`](docs/devices.md) | GUIDs et périphériques |
| [`docs/equalizer-apo.md`](docs/equalizer-apo.md) | Chaînes VST |
| [`docs/architecture.md`](docs/architecture.md) | Schémas et options historiques |
| [`docs/dev-mix-mixer.md`](docs/dev-mix-mixer.md) | Spec MixMixer v0.1 |
| [`docs/validate-mix-mixer.md`](docs/validate-mix-mixer.md) | Checklist validation |
| [`docs/decisions.md`](docs/decisions.md) | Journal ADR |
| [`mix-mixer/README.md`](mix-mixer/README.md) | Guide install / usage |

---

## Notes de session

### 2026-07-14 — MixMixer v0.1 baseline

- App simplifiée : **micro → VB-Cable** uniquement (plus de WAV hotkeys, plus de capture CABLE Output).
- UI Réglages : Appliquer / Annuler / Quitter / Activer-Désactiver + métriques temps réel.
- Reconnexion automatique des flux audio après changement de périphérique (Discord/GTA).
- Release sans terminal Windows.
- Dépôt Git local initialisé (`8c50c7d`).
