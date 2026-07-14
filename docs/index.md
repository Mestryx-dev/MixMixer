# Documentation canonique — Audio

> Index des documents de référence du projet.  
> **Point d'entrée agent :** [`../agent.md`](../agent.md)  
> **Version app :** MixMixer v0.1 baseline (`8c50c7d`)

---

## Convention

| Règle | Description |
|-------|-------------|
| **Source de vérité matérielle** | `devices.md` — GUIDs, noms Windows, état connecté/déconnecté |
| **Source de vérité E-APO** | `equalizer-apo.md` — chaînes VST, chemins config, backups |
| **Source de vérité MixMixer** | `dev-mix-mixer.md` + `mix-mixer/README.md` |
| **Audits datés** | `audit/YYYY-MM-DD.md` — snapshot read-only, jamais écrasé |
| **Décisions** | `decisions.md` — choix d'architecture (format ADR léger) |
| **Git local** | Dépôt `d:\Audio\` — pas de remote configuré |

Les audits sont **immutables** après rédaction. L'état vivant est dans `agent.md` et les docs de référence.

---

## Documents

| Document | Contenu |
|----------|---------|
| [audit/2026-07-14.md](audit/2026-07-14.md) | Premier audit local (read-only) |
| [devices.md](devices.md) | Inventaire périphériques + GUIDs |
| [equalizer-apo.md](equalizer-apo.md) | Config Equalizer APO + inventaire VST |
| [architecture.md](architecture.md) | Schémas, options historiques, MixMixer v0.1 |
| [dev-mix-mixer.md](dev-mix-mixer.md) | Spec technique MixMixer v0.1 |
| [validate-mix-mixer.md](validate-mix-mixer.md) | Checklist validation manuelle |
| [decisions.md](decisions.md) | Journal des décisions (DEC-001 … DEC-006) |
| [../mix-mixer/README.md](../mix-mixer/README.md) | Guide install / usage Windows |

---

## Arborescence

```
d:\Audio\
├── .git/                    ← dépôt local (baseline 8c50c7d)
├── agent.md                 ← hub agent (statut courant)
├── mix-mixer/               ← app MixMixer (Rust)
│   ├── src/
│   ├── config.json
│   └── target/              ← ignoré par git
└── docs/
    ├── index.md             ← ce fichier
    ├── dev-mix-mixer.md
    ├── validate-mix-mixer.md
    ├── architecture.md
    ├── decisions.md
    ├── devices.md
    ├── equalizer-apo.md
    └── audit/
        └── 2026-07-14.md
```
