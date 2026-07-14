# Documentation canonique — Audio

> Index des documents de référence du projet.  
> **Point d'entrée agent :** [`../agent.md`](../agent.md)

---

## Convention

| Règle | Description |
|-------|-------------|
| **Source de vérité matérielle** | `devices.md` — GUIDs, noms Windows, état connecté/déconnecté |
| **Source de vérité E-APO** | `equalizer-apo.md` — chaînes VST, chemins config, backups |
| **Audits datés** | `audit/YYYY-MM-DD.md` — snapshot read-only du système, jamais écrasé |
| **Décisions** | `decisions.md` — choix d'architecture (format ADR léger) |
| **Architecture** | `architecture.md` — schémas actuel vs cible, options techniques |

Les audits sont **immutables** après rédaction. Les mises à jour d'état vivent dans `agent.md` (section « État actuel ») et les docs de référence.

---

## Documents

| Document | Contenu |
|----------|---------|
| [audit/2026-07-14.md](audit/2026-07-14.md) | Premier audit local complet (read-only) |
| [devices.md](devices.md) | Inventaire périphériques + GUIDs |
| [equalizer-apo.md](equalizer-apo.md) | Config Equalizer APO + inventaire VST |
| [architecture.md](architecture.md) | Schémas, options, écarts objectifs |
| [dev-mix-mixer.md](dev-mix-mixer.md) | Spec technique MixMixer (DEC-005) |
| [validate-mix-mixer.md](validate-mix-mixer.md) | Checklist validation manuelle MixMixer |
| [decisions.md](decisions.md) | Journal des décisions |

---

## Arborescence

```
d:\Audio\
├── agent.md                 ← hub agent (contexte riche + statut)
├── mix-mixer/               ← app MixMixer (Rust)
└── docs/
    ├── index.md             ← ce fichier
    ├── devices.md
    ├── equalizer-apo.md
    ├── architecture.md
    ├── dev-mix-mixer.md
    ├── validate-mix-mixer.md
    ├── decisions.md
    └── audit/
        └── 2026-07-14.md
```
