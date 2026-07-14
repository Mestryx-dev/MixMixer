# Agent Audio — Soundboard & routage micro

> Hub agent — contexte riche et statut courant.  
> **Documentation canonique :** [`docs/index.md`](docs/index.md)  
> Dernière mise à jour : **2026-07-14**

---

## Objectifs

1. **Dupliquer le micro** vers 2 sorties (casque + enregistrement, ou 2 apps).
2. **Mixer / injecter** de l'audio (soundboard) **dans le signal micro** vu par Discord, jeux, OBS.
3. **Latence minimale** — pas de délai audible lors de la lecture soundboard.

---

## État actuel du système

> Audit read-only du 2026-07-14 — détail complet : [`docs/audit/2026-07-14.md`](docs/audit/2026-07-14.md)

| Élément | État |
|---------|------|
| **Micro actif** | fifine Microphone — E-APO (8 VST) uniquement |
| **Casque / sortie** | fifine SC3 — sans Equalizer APO |
| **Soundboard** | **MixMixer** (Rust/WASAPI) — MVP implémenté, validation manuelle en cours |
| **Soundpad** | Installé (Steam Demo) — **non lancé, UniteFx inactif** en local |
| **Câble virtuel** | VB-CABLE — installé, routé par MixMixer |
| **Mixeur** | **MixMixer** (remplace VoiceMeeter / Soundpad pour l'injection) |
| **Interface Behringer** | Pilote OK — **déconnectée** |
| **Duplication micro** | ✅ via MixMixer → CABLE Input + monitor SC3 |
| **Injection soundboard** | ✅ WAV hotkeys + CABLE Output (MixMixer) |

### Schéma cible (MixMixer — DEC-005)

```
Micro fifine → E-APO (8 VST) → MixMixer capture
CABLE Output → MixMixer capture (sfx externe)
WAV hotkeys  → MixMixer player
                    │
                    ├──► CABLE Input → Discord (micro = CABLE Output)
                    └──► fifine SC3 (monitor, défaut ON)
```

Référence devices : [`docs/devices.md`](docs/devices.md)  
Chaîne VST détaillée : [`docs/equalizer-apo.md`](docs/equalizer-apo.md)  
Architecture : [`docs/architecture.md`](docs/architecture.md) — section MixMixer  
Spec dev : [`docs/dev-mix-mixer.md`](docs/dev-mix-mixer.md)  
Validation : [`docs/validate-mix-mixer.md`](docs/validate-mix-mixer.md)  
Code : [`mix-mixer/`](mix-mixer/)

---

## Contexte technique

### Le problème Windows

Windows expose **un flux d'entrée** par micro. Pour injecter une soundboard :

- Il faut **mixer** micro + son injecté, **ou**
- Utiliser un APO qui **insère** l'audio dans le flux capture (Soundpad UniteFx).

Equalizer APO **traite** un flux ; il **ne mixe pas** deux sources. Le mixage passe par VoiceMeeter, un APO d'injection (Soundpad), ou une interface avec mix hardware.

### Ce qui est déjà en place

Tu n'es **pas parti de zéro** :

| Composant | Rôle |
|-----------|------|
| **Equalizer APO 1.4.2** | Chaîne voix pro (gate, EQ, saturation, bass, limiteurs) sur le micro fifine |
| **Soundpad Demo** | Installé Steam — **app non lancée, UniteFx inactif** (confirmé utilisateur 2026-07-14) |
| **MixMixer** | Mix voix + soundboard → CABLE Input + monitor SC3 |
| **VB-CABLE** | Micro virtuel pour Discord (CABLE Output) |
| **Behringer (config)** | Entrée ligne configurée en mono + Harmonic Maximizer + LoudMax — piste « jack » quand l'interface est branchée |

### Risque principal identifié

**Chaîne VST lourde** sur le micro fifine (8 plugins, dont 2× Limiter6 avec latence activée) :

```
Gate → Limiter → EQ → Saturation → IVGI → Mach3 → Harmonic → Limiter
```

Les deux **Limiter6** ont la latence activée (`Latency 1`). Premier levier si la soundboard MixMixer est tardive.

---

## Pistes techniques (résumé)

| Option | Description | Statut |
|--------|-------------|--------|
| **MixMixer (DEC-005)** | App Rust WASAPI — mix voix + sfx → CABLE Input + monitor | **✅ MVP implémenté** |
| **A — Soundpad seul** | UniteFx déjà installé | Remplacé par MixMixer |
| **C — VoiceMeeter** | Mix + duplication sans VST custom | Non retenu (trop lourd) |
| **2 — VST injecteur** | InjectMix fin de chaîne E-APO | Non retenu pour MVP |
| **E — Rack virtuel** | VoiceMeeter Potato ou app custom | Hors scope MVP |

Détail schémas : [`docs/architecture.md`](docs/architecture.md) — section **MixMixer (DEC-005)**.

**Décision acceptée (DEC-005) :** MixMixer pour injection soundboard + monitor. Voir [`docs/decisions.md`](docs/decisions.md).

---

## Inventaire rapide

### Micro fifine — chaîne VST active

| # | Plugin |
|---|--------|
| 1 | Sonalksis SV-719 Stereo Gate |
| 2 | Limiter6 |
| 3 | ReaEQ |
| 4 | ThrillseekerXTC mkIII |
| 5 | IVGI2 |
| 6 | Mach 3 Bass |
| 7 | Harmonic Maximizer |
| 8 | Limiter6 (2e instance) |

Config : `C:\Program Files\EqualizerAPO\config\config.txt`  
Backup : `D:\Nextcloud\NAS-Private\Réglage Micro\2026\config\config.txt`

### Logiciels

| App | Chemin |
|-----|--------|
| Equalizer APO | `C:\Program Files\EqualizerAPO\` |
| Soundpad Demo | `G:\SteamLibrary\steamapps\common\Soundpad\` |
| OBS Studio | `G:\SteamLibrary\steamapps\common\OBS Studio\` |

---

## Prochaines étapes

1. [ ] **Validation manuelle MixMixer** — checklist [`docs/validate-mix-mixer.md`](docs/validate-mix-mixer.md)
2. [ ] **Configurer Discord** — micro = CABLE Output
3. [ ] **Remplacer WAV placeholder** dans `mix-mixer/sounds/` par vrais SFX
4. [ ] **Vérifier UniteFx reste inactif** — si réactivé un jour, le désactiver (Device Selector) pour éviter double injection avec MixMixer
5. [ ] **Nouvel audit** — `docs/audit/YYYY-MM-DD.md` après validation terrain

---

## Questions ouvertes

| Question | Réponse audit 2026-07-14 |
|----------|--------------------------|
| Micro / interface ? | fifine Microphone + SC3 ; Behringer offline |
| E-APO capture ou playback ? | **Capture** (micro) + config Behringer playback/ligne (offline) |
| Soundboard ? | **MixMixer** (WAV + VB-Cable) ; Soundpad installé mais **inactif** |
| VoiceMeeter ? | **Non installé** |
| Entendre soundboard dans le casque ? | *À confirmer* |
| Latence max acceptable ? | *À confirmer* (cible ~20 ms ?) |
| Budget Soundpad full vs demo ? | *À confirmer* |

---

## Documentation canonique

| Document | Rôle |
|----------|------|
| [`docs/index.md`](docs/index.md) | Index et conventions |
| [`docs/audit/2026-07-14.md`](docs/audit/2026-07-14.md) | Snapshot audit (immutable) |
| [`docs/devices.md`](docs/devices.md) | GUIDs et périphériques |
| [`docs/equalizer-apo.md`](docs/equalizer-apo.md) | Chaînes VST, chemins, inventaire plugins |
| [`docs/architecture.md`](docs/architecture.md) | Schémas actuel/cible, options |
| [`docs/dev-mix-mixer.md`](docs/dev-mix-mixer.md) | Spec MixMixer |
| [`docs/validate-mix-mixer.md`](docs/validate-mix-mixer.md) | Checklist validation MixMixer |
| [`docs/decisions.md`](docs/decisions.md) | Journal des décisions (ADR) |
| [`mix-mixer/README.md`](mix-mixer/README.md) | Guide install / setup Windows |

**Convention :** mettre à jour `devices.md` / `equalizer-apo.md` après changement config ; créer un **nouveau** fichier `audit/YYYY-MM-DD.md` pour chaque re-scan système ; garder `agent.md` comme vue d'ensemble vivante.

---

## Notes de session

### 2026-07-14 (suite)

- **MixMixer MVP** implémenté dans `mix-mixer/` (Rust/cpal/WASAPI).
- Build release OK ; `--list-devices` confirme fifine, CABLE, SC3 à 48 kHz.
- DEC-005 accepté — remplace VoiceMeeter / Soundpad pour l'injection soundboard.
- **Correction utilisateur :** Soundpad non actif en local (app arrêtée, UniteFx inactif) — pas de double APO à gérer actuellement.
