# Architecture audio

> Schémas actuel vs cible. Options techniques pour soundboard + duplication micro.

---

## État actuel (2026-07-14)

```
┌─────────────────────────────────────────────────────────────────┐
│                     MICRO FIFINE (physique)                      │
└───────────────────────────────┬─────────────────────────────────┘
                                │
                    ┌───────────▼───────────┐
                    │  UniteFx (Soundpad)   │  ← installé, app arrêtée
                    └───────────┬───────────┘
                                │
                    ┌───────────▼───────────┐
                    │   Equalizer APO       │
                    │   8× VST (gate→lim)   │
                    └───────────┬───────────┘
                                │
                    ┌───────────▼───────────┐
                    │ Discord / OBS / jeux  │
                    └───────────────────────┘

┌──────────────┐     ┌──────────────┐     ┌──────────────────────┐
│  VB-Cable    │     │  Behringer   │     │  fifine SC3 (HP)     │
│  (isolé)     │     │  (offline)   │     │  pas d'E-APO         │
└──────────────┘     └──────────────┘     └──────────────────────┘
```

### Constats

- Pas de mixeur → impossible de mixer micro + VB-Cable sans outil additionnel.
- Soundpad peut injecter **sans** VoiceMeeter, mais partage le pipeline avec E-APO.
- Behringer entrée ligne = config prête pour injection « jack » quand reconnecté.

---

## Objectifs

1. Dupliquer le micro vers 2 sorties.
2. Injecter soundboard dans le signal micro (Discord, jeux, OBS).
3. Latence minimale sur la soundboard.

---

## Options évaluées

### A — Soundpad seul (état quasi actuel)

```
Micro → UniteFx (Soundpad injecte ici) → E-APO VST → apps
```

| Pour | Contre |
|------|--------|
| Déjà installé | Demo Steam (limites) |
| Pas de mixeur requis | Latence = UniteFx + 8 VST |
| Injection native micro | Pas de duplication sortie |

**Action minimale :** lancer Soundpad, tester latence, éventuellement alléger VST.

---

### B — Behringer entrée ligne + E-APO

```
Micro fifine ──────────────────────────► mix ? ──► apps
Soundboard → Behringer Line In → E-APO (Harmonic+LoudMax) ──┘
```

| Pour | Contre |
|------|--------|
| Config déjà partielle dans `configAPO.txt` | Behringer offline ; mix micro+ligne non trivial dans E-APO seul |
| Entrée jack physique | Equalizer APO ne mixe pas 2 captures |

**Limite :** E-APO traite **un device par config** — le mix micro + ligne nécessite VoiceMeeter ou mix hardware Behringer.

---

### C — VoiceMeeter + E-APO ⭐ (recommandé si Soundpad seul insuffisant)

```
Micro ──► VM Input 1 ──┐
Soundpad / app ──► VM Input 2 ──┤
                                ▼
                          VM Mix (A1 + B1)
                                │
              ┌─────────────────┴─────────────────┐
              ▼                                   ▼
        Casque (A1)                    VM Output = micro virtuel
              │                                   │
              │                          E-APO (optionnel, chaîne VST)
              │                                   │
              └──────────────► Discord / OBS ◄────┘
```

| Pour | Contre |
|------|--------|
| Mix + duplication native | Installation + courbe d'apprentissage |
| VB-Cable déjà présent | Latence supplémentaire (~10–20 ms) |
| E-APO en aval du mix | Reconfig devices Windows |

---

### D — Développement custom (VST / WASAPI)

Envisager seulement si A–C échouent. Voir `agent.md` section dev.

---

## Deux options en débat (2026-07-14)

### Option 1 — Dupliquer le micro vers VB-Cable

**Objectif :** envoyer une copie du signal micro vers `CABLE Input` pour qu'une autre app lise `CABLE Output`.

```
Micro fifine ──► [E-APO 8 VST] ──► Discord / jeux (micro Windows)
                      │
                      └──► copie ──► CABLE Input ──► CABLE Output ──► OBS / enregistreur / 2e app
```

| Pour | Contre |
|------|--------|
| 2 apps consomment le même flux (ex. Discord + OBS) | **E-APO ne duplique pas** — il faut un routeur |
| VB-Cable déjà installé | Sans VoiceMeeter ou app WASAPI custom : pas trivial |
| Pas de dev VST | Ne **injecte pas** dans le micro — branche parallèle seulement |

**Implémentations possibles :**

| Méthode | Latence | Dev |
|---------|---------|-----|
| VoiceMeeter (micro → B1 = CABLE) | ~10–20 ms | Config |
| App WASAPI loopback → CABLE Input | Contrôlable | Moyen |
| Windows « Écouter ce périphérique » | Élevée | ❌ déconseillé |

**Limite :** si la copie part **avant** E-APO, OBS n'entend pas la chaîne VST. Si **après**, il faut capturer le device micro déjà traité et le réinjecter vers CABLE — boucle ou tap WASAPI.

---

### Option 2 — VST injecteur en fin de chaîne (2e source : micro ou VB-Cable)

**Objectif :** garder **un seul** micro Windows ; mixer en **dernier** plugin ce qui arrive d'une 2e source.

```
Source A (voix)     Micro fifine ──► Gate → EQ → … → [InjectMix VST] ──► apps
Source B (inject)   VB-Cable Output ──► app compagnon (WASAPI) ──► ring buffer ──► ↑
                    ou 2e micro ──► app compagnon ──► ring buffer ──► ↑
```

| Pour | Contre |
|------|--------|
| Soundboard **sèche** en fin de chaîne (gate ne coupe pas les sfx) | Dev VST + app compagnon |
| Un seul device micro pour Discord | Le VST ne lit pas VB-Cable directement — **app intermédiaire** requise |
| Remplace Soundpad UniteFx (1 APO en moins) | Latence = capture 2e source + ring buffer |

**Flux VB-Cable typique :**

```
Soundboard app ──► CABLE Input (playback)
                        │
                   CABLE Output (capture)
                        │
              App compagnon (WASAPI capture, ~128–512 buffer)
                        │
              Ring buffer (shared memory)
                        │
              InjectMix VST (dernier dans config.txt)
```

**Flux 2e micro :** idem, l'app compagnon capture le 2e micro au lieu de CABLE Output.

**Config E-APO (extrait) :**

```txt
Device: Microphone fifine Microphone {c63663e8-...}
VSTPlugin: Library "Sonalksis SV-719 Stereo Gate (64 bit).dll" ...
# … chaîne voix existante …
VSTPlugin: Library "InjectMix-x64.dll" InjectGain 0.85
```

---

### Comparaison directe

| Critère | Option 1 — Dupliquer → VB-Cable | Option 2 — VST fin de chaîne |
|---------|--------------------------------|------------------------------|
| **Besoin principal injection Discord** | ❌ indirect | ✅ direct |
| **Dupliquer vers 2e app (OBS)** | ✅ naturel | ⚠️ possible mais pas le focus |
| **Dev custom** | Optionnel (WASAPI router) | **Requis** (VST + app) |
| **VoiceMeeter suffit ?** | Oui, sans dev | Non (mix **dans** E-APO) |
| **Soundboard via VB-Cable** | Branche parallèle | ✅ source B idéale |
| **Latence soundboard** | Dépend du routeur | Faible si WAV pré-décodés + buffer court |

**Recommandation :**

- **Injection soundboard sans délai** → **Option 2** (alignée objectif principal).
- **Dupliquer le micro traité vers OBS en parallèle** → **Option 1** (VoiceMeeter ou tap WASAPI).
- Les deux sont **combinables** : Option 2 pour Discord + Option 1 si OBS doit aussi enregistrer le mix.

---

## Option E — Rack / mixer virtuel (router tous les flux)

**Question :** peut-on avoir un **système de rack** pour envoyer n'importe quelle source vers n'importe quelle sortie ?

**Réponse : oui.** C'est la définition d'un **mixeur matriciel** logiciel. Trois niveaux possibles :

### E1 — VoiceMeeter Potato (rack clé en main) ⭐

VoiceMeeter **Banana** (3 entrées / 3 sorties virtuelles) ou **Potato** (5 / 5) = déjà un rack :

```
┌─────────────────────────────────────────────────────────────┐
│                    VOICEMEETER POTATO (rack)                 │
├──────────────┬──────────────┬──────────────┬────────────────┤
│  Strip 1     │  Strip 2     │  Strip 3     │  Virtual I/O   │
│  Micro fifine│  VB-Cable    │  Behringer   │  VAIO / AUX    │
│  Hardware In1│  (soundboard)│  ligne       │  (apps)        │
├──────────────┴──────────────┴──────────────┴────────────────┤
│  Bus A1 → Casque SC3    Bus A2 → HP Behringer               │
│  Bus B1 → VB-Cable Out (= micro virtuel Discord)            │
│  Bus B2 → OBS / enregistrement                              │
│  Niveaux · Pan · Mute · Solo · EQ intégré (basique)         │
└─────────────────────────────────────────────────────────────┘
         │                                    │
         ▼                                    ▼
   Equalizer APO (optionnel)            Apps Windows
   sur le bus B1 ou strip 1            (micro = VM Output)
```

| Route exemple | Réglage VM |
|---------------|------------|
| Micro → Discord + casque | Strip1 → B1 + A1 |
| Soundboard → Discord seulement | Strip2 → B1 (pas A1 si tu ne veux pas l'entendre) |
| Micro + sfx → OBS | Strip1+2 → B2 |
| Dupliquer micro traité E-APO | E-APO sur strip1 **ou** sur B1 en aval |

**Où mettre Equalizer APO :**
- **Sur le micro physique** (actuel) — tout ce qui passe strip1 est traité.
- **Sur VoiceMeeter Output (B1)** — seul le mix envoyé à Discord est traité ; soundboard peut bypass la chaîne voix.

**Latence :** ~10–25 ms. Suffisant pour soundboard si buffers bas.

---

### E2 — REAPER + ReaRoute (rack pro / studio)

REAPER agit comme rack illimité : pistes, sends, returns, VST par piste, ReaRoute vers devices Windows.

| Pour | Contre |
|------|--------|
| Rack visuel complet, VST illimités | Lourd, orienté DAW |
| Routage arbitraire | Latence + complexité |
| Tu connais déjà l'écosystème VST | Overkill pour « parler sur Discord » |

---

### E3 — Rack custom maison (dev)

Un **FlowAudio Rack** — app dédiée avec matrice source × destination :

```
┌────────────────────────────────────────────────────────────┐
│  SOURCES (inputs)          │  MATRICE        │  DESTINATIONS │
├──────────────────────────┼─────────────────┼───────────────┤
│ · Micro fifine           │  [x] gain/mute  │ · VM / VB-Cable│
│ · VB-Cable Output        │  par cellule    │ · Casque SC3   │
│ · Behringer line         │                 │ · OBS bus      │
│ · Soundboard (WAV)       │  VST insert   │ · « Micro » virt│
│ · App loopback (WASAPI)  │  par strip      │               │
└──────────────────────────┴─────────────────┴───────────────┘
```

**Stack possible :**
- **Rust** : `cpal` + `rubato` + egui (UI matrice)
- **C++/JUCE** : AudioDeviceManager + matrix mixer + VST host optionnel
- **IPC** : pas obligatoire si tout est dans une seule app

**Difficultés Windows :**
- Capturer **une app précise** (Discord, jeu) → WASAPI session capture ou loopback device
- **Plusieurs devices** simultanés → sync horloge (drift) entre interfaces
- **Low latency** → buffers 128–512, même sample rate partout (48 kHz)

**Scope MVP rack custom :**
1. 4 entrées × 4 sorties, gains + mute
2. 1 bus master « micro virtuel » (VB-Cable Output)
3. Soundboard intégrée (hotkeys WAV) — remplace Soundpad + Option 2 VST
4. Option : lien avec E-APO (traitement sur un bus, pas sur tout)

---

### Comparatif rack

| Solution | Router tout | Dev | Latence | E-APO compatible |
|----------|-------------|-----|---------|------------------|
| **VoiceMeeter Potato** | ✅ quasi tout | ❌ config | ~15 ms | ✅ par device/bus |
| **REAPER + ReaRoute** | ✅ tout | ❌ config | Variable | ✅ inserts VST natifs |
| **Rack custom** | ✅ sur mesure | ⭐⭐⭐ mois | Optimisable | ✅ si 1 bus → device E-APO |
| **Option 2 seule (VST inject)** | ❌ 1 mix seulement | ⭐⭐ semaines | Basse | ✅ dans la chaîne |

---

### Recommandation rack vs options 1/2

| Ton besoin | Meilleure piste |
|------------|-----------------|
| « Je veux router **tout** où je veux » | **VoiceMeeter Potato** (E1) — immédiat |
| « Je veux **uniquement** injecter sfx en fin de chaîne E-APO » | **Option 2** (VST InjectMix) |
| « Je veux **mon** rack avec UI perso + Stream Deck » | **Rack custom** (E3) — long terme |
| « Studio / enregistrement complexe » | REAPER (E2) |

**Architecture hybride recommandée** (le plus pragmatique pour toi) :

```
[Rack VoiceMeeter]
  In1 = Micro fifine
  In2 = VB-Cable (soundboard)
  In3 = Behringer ligne (quand branché)
  B1  = sortie « micro Discord » → Equalizer APO (chaîne VST actuelle) → apps
  A1  = casque SC3 (monitor)
  B2  = OBS
```

→ Tu obtiens **rack + chaîne E-APO + soundboard + duplication** sans dev VST.  
→ Option 2 (VST inject) reste utile **seulement** si tu refuses VoiceMeeter et veux tout dans E-APO.

---

---

## MixMixer — solution retenue (DEC-005)

App Rust tray minimaliste (`mix-mixer/`) qui remplace VoiceMeeter et Soundpad UniteFx pour l'injection soundboard.

```
┌─────────────────────────────────────────────────────────────────┐
│                     MICRO FIFINE (physique)                      │
└───────────────────────────────┬─────────────────────────────────┘
                                │
                    ┌───────────▼───────────┐
                    │   Equalizer APO       │
                    │   8× VST (gate→lim)   │
                    └───────────┬───────────┘
                                │
                    ┌───────────▼───────────┐
                    │      MixMixer         │
                    │  capture + mix + sfx  │
                    └───────────┬───────────┘
                                │
              ┌─────────────────┴─────────────────┐
              ▼                                   ▼
    CABLE Input → CABLE Output          fifine SC3 (monitor)
    (Discord / jeux / OBS)              (casque, défaut ON)

CABLE Output ◄── capture ◄── apps externe (playback → CABLE Input)
WAV hotkeys  ◄── player interne
```

| Pour | Contre |
|------|--------|
| Un seul exe tray, pas VoiceMeeter | Pas de matrice N×M (hors scope MVP) |
| Voix post-E-APO + sfx WAV + VB-Cable | Validation manuelle Discord requise |
| Monitor SC3 intégré | Redémarrage pour changer devices |
| Latence ~15–35 ms estimée | Resampling si device ≠ 48 kHz |

**Setup :** voir [`mix-mixer/README.md`](../mix-mixer/README.md) et [`docs/dev-mix-mixer.md`](dev-mix-mixer.md).

---

## Matrice décision rapide

| Besoin | Option privilégiée |
|--------|-------------------|
| **Injection soundboard + monitor (actuel)** | **MixMixer** — DEC-005 |
| Tester vite, soundboard seule | ~~A — Soundpad~~ → MixMixer |
| Dupliquer micro + mix propre | **C / E1** — VoiceMeeter Potato |
| Router tous les flux (rack) | **E1** — VoiceMeeter Potato, ou **E3** dev custom |
| Contrôle total latence | **MixMixer** (WASAPI direct) |

---

## Checklist latence

- [ ] Soundboard locale (fichiers disque, pas URL web)
- [ ] Soundpad lancé avant les apps vocales
- [ ] Réduire chaîne VST (retirer 2e Limiter6, désactiver `Latency` sur Limiter6)
- [ ] 48 kHz identique partout
- [ ] Éviter « Écouter ce périphérique » Windows
- [ ] VoiceMeeter buffers 128–512 si option C
