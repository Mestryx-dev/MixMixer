# Inventaire périphériques

> Référence canonique des devices Windows. Mettre à jour après changement matériel.  
> Dernier sync audit : **2026-07-14**

---

## Périphériques dans la chaîne vocale

### Micro principal — fifine Microphone

| Propriété | Valeur |
|-----------|--------|
| Nom Windows | `Microphone (2- fifine Microphone)` |
| GUID capture | `{c63663e8-2d39-403c-bedb-35b70bb4c6b0}` |
| USB | `VID_3142&PID_0068` |
| Equalizer APO | ✅ Oui — `config.txt` |
| Soundpad UniteFx | ⚠️ DLL présente — **inactif** (confirmé utilisateur 2026-07-14) |
| État (2026-07-14) | Connecté |

### Sortie casque — fifine SC3

| Propriété | Valeur |
|-----------|--------|
| Nom Windows | `Haut-parleurs (fifine SC3)` |
| GUID render | `{5c7a2cdc-441e-415b-a5b7-771551f68223}` |
| USB | `VID_3142&PID_0C33` |
| Equalizer APO | ❌ Non (FX Realtek / Windows par défaut) |
| État (2026-07-14) | Connecté |

---

## Câble virtuel — VB-Audio

| Rôle | Nom Windows | GUID | E-APO |
|------|-------------|------|-------|
| Playback (injecter du son) | CABLE Input | `{ec8a141e-1340-4497-b2b0-62df89ec80b1}` | ❌ |
| Capture (lire le mix) | CABLE Output | `{5f5f2573-25a5-49aa-9410-e4e789159e73}` | ❌ |
| Playback 16ch | CABLE In 16ch | `{ac0ede3b-a006-432d-9b4c-2d94b247653b}` | ❌ |

**Usage actuel :** installé, non routé dans la chaîne micro.

---

## Interface Behringer (hors ligne)

| Rôle | Nom Windows | GUID | E-APO |
|------|-------------|------|-------|
| Sortie HP | Haut-parleurs BEHRINGER USB WDM AUDIO 2.8.40 | `{0859fed2-533f-4b6c-bbad-b6e33c5eab2a}` | ✅ `configAPO.txt` |
| Entrée ligne | Entrée ligne BEHRINGER USB WDM AUDIO 2.8.40 | `{f2974733-5001-4949-a73c-9e2d313a73fc}` | ✅ `configAPO.txt` |

**État (2026-07-14) :** pilote installé, **interface non connectée**.

**Intérêt projet :** entrée ligne = piste « jack » pour router soundboard ou ligne externe, avec traitement E-APO (mono + mastering).

---

## Autres périphériques (non chaîne vocale)

| Appareil | Notes |
|----------|-------|
| Fifine K420 | Webcam USB, audio disponible |
| Realtek HD Audio | Carte mère |
| NVIDIA HD Audio | Sorties HDMI |
| Steam Streaming Microphone / Speakers | Virtuels Valve |
| Nahimic mirroring device | Mirroring Realtek |

---

## Logiciels associés

| App | Chemin | Lien device |
|-----|--------|-------------|
| Soundpad Demo | `G:\SteamLibrary\steamapps\common\Soundpad\` | UniteFx sur micro fifine |
| OBS Studio | `G:\SteamLibrary\steamapps\common\OBS Studio\` | Capture micro Windows |
| Equalizer APO | `C:\Program Files\EqualizerAPO\` | Micro fifine + Behringer (offline) |
