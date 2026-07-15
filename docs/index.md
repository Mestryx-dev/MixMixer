# Documentation index — MixMixer / Audio

> Entry point for agents: [`../agent.md`](../agent.md)  
> **App version:** MixMixer **v0.1.4-beta.1**  
> **Public repo:** [github.com/Mestryx-dev/MixMixer](https://github.com/Mestryx-dev/MixMixer)

---

## Public vs internal

| Kind | Language | Audience |
|------|----------|----------|
| **Public** | English | GitHub users, releases, setup |
| **Internal** | French (mostly) | Local machine notes, ADRs, hardware inventory |

---

## Public documents (English)

| Document | Content |
|----------|---------|
| [../README.md](../README.md) | Product overview, config reference, screenshot |
| [TUTORIAL.md](TUTORIAL.md) | Step-by-step Windows setup |
| [RELEASE-v0.1.4-beta.1.md](RELEASE-v0.1.4-beta.1.md) | Current (beta) release notes / install |
| [../CHANGELOG.md](../CHANGELOG.md) | Version history |
| [../CONTRIBUTING.md](../CONTRIBUTING.md) | How to contribute |
| [images/settings-window.png](images/settings-window.png) | Settings UI screenshot |

---

## Internal documents

| Document | Content |
|----------|---------|
| [dev-mix-mixer.md](dev-mix-mixer.md) | Technical spec (UI, tray/Windows threading) |
| [validate-mix-mixer.md](validate-mix-mixer.md) | Manual validation checklist |
| [architecture.md](architecture.md) | Historical options + MixMixer flow |
| [decisions.md](decisions.md) | ADR journal (DEC-001 …) |
| [devices.md](devices.md) | Local device inventory + GUIDs |
| [equalizer-apo.md](equalizer-apo.md) | Local E-APO / VST chain notes |
| [audit/2026-07-14.md](audit/2026-07-14.md) | Dated read-only audit snapshot |

Audits are **immutable** after writing. Living status stays in `agent.md`.

---

## Conventions

| Rule | Description |
|------|-------------|
| Hardware source of truth | `devices.md` |
| E-APO source of truth | `equalizer-apo.md` |
| MixMixer product docs | Root `README.md` + `TUTORIAL.md` |
| Screenshots | `docs/images/` |
| User config | Never commit `mix-mixer/config.json` |

---

## Tree (high level)

```
Audio/
├── README.md                 ← public product page
├── CHANGELOG.md
├── LICENSE
├── CONTRIBUTING.md
├── agent.md                  ← agent hub (internal)
├── mix-mixer/                ← Rust app
│   ├── config.example.json
│   └── src/
└── docs/
    ├── index.md              ← this file
    ├── TUTORIAL.md
    ├── RELEASE-v0.1.4-beta.1.md
    ├── images/
    │   └── settings-window.png
    └── … internal notes …
```
