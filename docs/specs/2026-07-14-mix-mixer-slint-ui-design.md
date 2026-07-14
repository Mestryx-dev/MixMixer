# MixMixer v0.2 — Migration UI Slint (remplacement complet egui)

**Date:** 2026-07-14  
**Statut:** Proposition — en attente validation  
**Décisions utilisateur:** Slint natif, remplacement complet (settings + tray), look Apple/iOS Settings

---

## Contexte

MixMixer v0.1 utilise **egui/eframe** pour la fenêtre Réglages. Le moteur audio (cpal, WASAPI, crossbeam) est stable. L’UI egui ne permet pas un rendu Apple convaincant malgré plusieurs itérations (`theme.rs`).

**Stack actuelle (UI):**

| Composant | Rôle |
|-----------|------|
| `eframe` + `egui` | Fenêtre settings (thread séparé) |
| `tray-icon` | Icône + menu contextuel |
| `winit` | Backend fenêtre egui |
| `theme.rs` / `settings.rs` | Style + logique formulaire |

**Objectif v0.2:** Remplacer entièrement cette couche par **Slint 1.17+** sans toucher au moteur audio.

---

## Pourquoi Slint (vs Tauri)

| Critère | Slint | Tauri |
|---------|-------|-------|
| Rendu Apple via CSS | Moyen (DSL custom) | Excellent (HTML/CSS) |
| Binaire / RAM | ~5–15 Mo, natif | ~80 Mo (WebView) |
| Rust pur | Oui | Backend Rust + frontend JS |
| Tray natif | `SystemTrayIcon` (1.17) | `tray-icon` feature |
| Build | `rustc` + slint-build | Rust + Node.js |
| Latence audio | Pas de WebView, IPC minimal | IPC WebView ↔ Rust |

**Choix retenu:** Slint — aligné avec la demande utilisateur, binaire léger, cohérent avec un utilitaire tray audio bas latence.

---

## Architecture cible

```
┌─────────────────────────────────────────────────────────────┐
│                    Thread principal (Slint)                  │
│  ┌──────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │ SystemTray   │  │ SettingsWindow  │  │ Timer 16ms      │ │
│  │ Icon (.slint)│  │ (AppWindow)     │  │ poll AppEvent   │ │
│  └──────┬───────┘  └────────┬────────┘  └────────┬────────┘ │
│         │                   │                     │          │
│         └───────────────────┴─────────────────────┘          │
│                             │ crossbeam AppEvent              │
└─────────────────────────────┼─────────────────────────────────┘
                              │
┌─────────────────────────────▼─────────────────────────────────┐
│                    Thread audio (inchangé)                     │
│  AudioEngine — cpal streams, metrics, auto-reconnect           │
└───────────────────────────────────────────────────────────────┘
```

### Crates / fichiers

```
mix-mixer/
├── build.rs                    # slint-build compile ui/*.slint
├── ui/
│   ├── theme.slint             # couleurs iOS dark, typo, spacing
│   ├── components/
│   │   ├── section.slint       # titre + footer caption
│   │   ├── group.slint         # carte arrondie #1C1C1E
│   │   ├── toggle-row.slint    # label + Switch
│   │   ├── picker-row.slint    # label + ComboBox + chevron
│   │   └── slider-row.slint    # label + valeur + Slider pleine largeur
│   ├── settings-window.slint   # fenêtre principale
│   └── tray.slint              # SystemTrayIcon + menu
├── src/
│   ├── main.rs                 # init Slint, timer events, lancement audio
│   ├── app/
│   │   ├── mod.rs
│   │   ├── state.rs            # SettingsDraft, binding Slint ↔ Config
│   │   └── bridge.rs           # callbacks apply/cancel/routing/metrics
│   ├── audio/                  # INCHANGÉ
│   ├── config.rs               # INCHANGÉ
│   ├── devices.rs              # INCHANGÉ
│   └── ui/                     # SUPPRIMÉ (egui theme/settings/tray)
```

### Dépendances

**Ajouter:**

```toml
slint = "1.17"
# build-dependencies
slint-build = "1.17"
```

**Retirer:**

```toml
eframe, egui, winit
tray-icon   # remplacé par Slint SystemTrayIcon si stable sur Windows
```

---

## Design UI (Apple iOS Settings)

### Tokens (`theme.slint`)

```slint
export global Theme {
    in-out property <color> bg: #000000;
    in-out property <color> group: #1c1c1e;
    in-out property <color> separator: #38383a;
    in-out property <color> text: #ffffff;
    in-out property <color> text-secondary: #aeaeb2;
    in-out property <color> text-tertiary: #76767a;
    in-out property <color> accent: #0a84ff;
    in-out property <color> green: #30d158;
    in-out property <length> inset: 16px;
    in-out property <length> row-h: 44px;
    in-out property <length> group-radius: 12px;
}
```

### Composants std-widgets Slint

| Contrôle iOS | Slint |
|--------------|-------|
| UISwitch | `Switch` (std-widgets) |
| Picker row | `ComboBox` dans row custom |
| Slider | `Slider` pleine largeur sous le label |
| Grouped section | `Rectangle` + `border-radius` |
| Separator inset | `Rectangle` height 1px, margin-left 16px |

### Fenêtre Réglages

- **480×680** px, non redimensionnable horizontalement, scroll vertical si besoin (`Flickable` ou `ScrollView`)
- **Header:** MixMixer + métriques live (timer Rust → property `metrics-text`)
- **Sections:** ROUTAGE, PÉRIPHÉRIQUES, AUDIO (même contenu qu’aujourd’hui)
- **Footer:** Appliquer (accent) · Annuler · Quitter

### Tray

Slint 1.17 `SystemTrayIcon`:

- Menu: Réglages, Écoute on/off, Recharger config, Quitter
- Double-clic → ouvrir settings
- Tooltip: `MixMixer — micro → VAC`

**Fallback:** Si `SystemTrayIcon` Windows pose problème en POC, garder `tray-icon` temporairement branché sur le timer Slint (documenté DEC-007).

---

## Flux de données

### Properties Slint (Rust → UI)

```rust
// Exemple binding
ui.set_devices_inputs(ModelRc::new(StringModel::from(devices.inputs)));
ui.set_routing_enabled(config.enabled);
ui.set_metrics_text(format!("Actif · {:.1} ms · {:.0} %", ...));
```

### Callbacks Slint (UI → Rust)

| Callback | Action |
|----------|--------|
| `apply-settings` | Valide draft → `Config::save` → `AppEvent::SettingsApplied` |
| `cancel-settings` | Reset draft depuis baseline |
| `toggle-routing` | Immédiat (comme v0.1) → `SetRoutingEnabled` |
| `quit-app` | `AppEvent::Shutdown` |
| `open-settings` | `settings-window.show()` |
| tray menu items | Équivalent `TrayAction` actuel |

### Boucle événements

Remplacer `run_event_loop` + `thread::sleep(16ms)` par:

1. `slint::Timer` périodique (16 ms) → drain `event_rx`, mettre à jour properties
2. `ui.run()` bloque sur le thread principal
3. Thread audio reste identique

---

## Plan de migration (remplacement complet)

### Phase 1 — Scaffold Slint (1–2 h)

- [ ] `build.rs` + `ui/theme.slint` + fenêtre vide
- [ ] Compiler sans egui en parallèle (feature flag `slint-ui` optionnel)

### Phase 2 — Settings window (3–4 h)

- [ ] Composants iOS (group, toggle, picker, slider)
- [ ] `app/state.rs` — draft/baseline, device lists
- [ ] Brancher apply/cancel/routing sur crossbeam existant
- [ ] Metrics live via timer

### Phase 3 — Tray Slint (1–2 h)

- [ ] `tray.slint` SystemTrayIcon + menu
- [ ] Supprimer `tray-icon` si OK Windows
- [ ] Double-clic, actions menu

### Phase 4 — Nettoyage (1 h)

- [ ] Supprimer `eframe`, `egui`, `winit`, `src/ui/theme.rs`, `settings.rs`
- [ ] Mettre à jour README, `dev-mix-mixer.md`, DEC-007
- [ ] Validation manuelle checklist

**Estimation totale:** ~8 h

---

## Risques et mitigations

| Risque | Mitigation |
|--------|------------|
| SystemTrayIcon Windows immature | Fallback `tray-icon` + timer Slint |
| Slint ComboBox pas assez « iOS » | Custom `TouchArea` + `PopupWindow` |
| Thread principal bloqué | Timer + `invoke_from_event_loop` pour audio callbacks |
| Régression latence | Ne pas toucher `audio/engine.rs` |

---

## Critères de succès

1. Fenêtre Réglages visuellement proche iOS Settings (groupes, switches, pickers, sliders pleine largeur)
2. Aucun trait blanc / chevron inversé / slider tronqué
3. Tray fonctionnel (menu + double-clic)
4. Apply / routing / metrics identiques v0.1
5. Binaire release < 25 Mo, pas de Node.js au build
6. egui complètement retiré

---

## DEC-007 (proposé)

**Décision:** Migrer l’UI MixMixer de egui vers Slint 1.17 (remplacement complet).

**Raison:** egui ne permet pas un rendu Apple professionnel; Slint offre widgets natifs, tray intégré, binaire léger, stack Rust pure.

**Conséquences:** Suppression eframe/egui/winit/tray-icon; nouveau dossier `ui/*.slint`; boucle événements basée sur Slint.

---

## Prochaine étape

Après validation de ce spec → plan d’implémentation détaillé (writing-plans) puis migration Phase 1.
