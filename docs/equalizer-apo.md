# Equalizer APO — référence config

> Source de vérité pour les chaînes VST et chemins de configuration.

---

## Chemins

| Élément | Chemin |
|---------|--------|
| Installation | `C:\Program Files\EqualizerAPO\` |
| Config principale | `C:\Program Files\EqualizerAPO\config\config.txt` |
| Config devices additionnels | `C:\Program Files\EqualizerAPO\config\configAPO.txt` |
| Plugins VST (utilisés par E-APO) | `C:\Program Files\EqualizerAPO\VSTPlugins\` |
| Device Selector | `C:\Program Files\EqualizerAPO\DeviceSelector.exe` |
| Editor | `C:\Program Files\EqualizerAPO\Editor.exe` |

### Backups utilisateur

| Backup | Chemin | Contenu |
|--------|--------|---------|
| Config 2026 | `D:\Nextcloud\NAS-Private\Réglage Micro\2026\config\config.txt` | Config micro |
| VST 2026 (complet) | `D:\Nextcloud\NAS-Private\Réglage Micro\2026\VSTPlugins\` | Copie intégrale chaîne active |
| VST 2025 | `D:\Nextcloud\NAS-Private\Réglage Micro\2025\` | IVGI2, Limiter6, LoudMax, ReaEQ, Thrillseeker |
| VST divers | `D:\Nextcloud\NAS-Private\Réglage Micro\Plugins\` | Sous-ensemble (Gate, Harmonic, etc.) |
| Legacy Ableton | `D:\Nextcloud\Documents\Mes Documents\Plugin Ableton 9 ESSB\` | Anciens bundles Sonalksis, BBE, ReaPlugs |
| BBE originaux | `C:\Program Files\Steinberg\VSTPlugins\BBE Sound\` | Harmonic Maximizer, Mach 3 Bass |
| Presets BBE | `D:\Nextcloud\Documents\Mes Documents\BBE Sound\Sonic Sweet\` | Presets utilisateur |

---

## Chaîne active — Micro fifine

**Fichier :** `config.txt`  
**Device :** `Microphone fifine Microphone {c63663e8-2d39-403c-bedb-35b70bb4c6b0}`

| # | Plugin | Fichier | Rôle |
|---|--------|---------|------|
| 1 | Sonalksis SV-719 Stereo Gate | `Sonalksis SV-719 Stereo Gate (64 bit).dll` | Gate / noise |
| 2 | Limiter6 | `Limiter6-x64.dll` | Compresseur / limiteur |
| 3 | ReaEQ | `reaeq-standalone.dll` | EQ paramétrique (Cockos) |
| 4 | ThrillseekerXTC mkIII | `ThrillseekerXTCmkIII (64).dll` | Saturation |
| 5 | IVGI2 | `IVGI2.dll` | Saturation légère (Klanghelm) |
| 6 | Mach 3 Bass | `Mach 3 Bass.dll` | Bass enhancement (BBE) |
| 7 | Harmonic Maximizer | `Harmonic Maximizer.dll` | Exciter / harmoniques (BBE) |
| 8 | Limiter6 | `Limiter6-x64.dll` | Limiteur final (2e instance) |

**Note latence :** les deux Limiter6 ont `Latency 1` dans la config — suspect principal si délai soundboard perceptible.

---

## Chaîne Behringer — configAPO.txt (offline)

### Haut-parleurs Behringer `{0859fed2-533f-4b6c-bbad-b6e33c5eab2a}`

- T-De-Esser — **commenté**
- Include SteelSeries Arctis Pro EQ — **commenté** (fichier absent : `Steelseries EQ\...`)
- Filtre actif : **LP Fc 224.4 Hz**

### Entrée ligne Behringer `{f2974733-5001-4949-a73c-9e2d313a73fc}`

| Élément | Détail |
|---------|--------|
| `Copy: L=L+R R=L+R` | Somme mono L+R |
| Sonalksis SV-315 Compressor | **commenté** |
| PhaseNudge64 | **commenté** |
| Harmonic Maximizer | ✅ actif |
| LoudMax64 | ✅ actif |

**Usage prévu :** passer une source externe (soundboard, jack) par l'entrée ligne Behringer avec mastering léger.

---

## Inventaire VST installés (dossier E-APO)

Plugins présents dans `VSTPlugins\` (non tous utilisés dans la config active) :

| Plugin | Fichier |
|--------|---------|
| Sonalksis SV-719 Gate | `Sonalksis SV-719 Stereo Gate (64 bit).dll` |
| Sonalksis Uber Compressor | `Sonalksis Uber Compressor Mono (64 bit).dll` |
| Limiter6 | `Limiter6-x64.dll` |
| ReaEQ / ReaGate | `reaeq-standalone.dll`, `reagate-standalone.dll` |
| ThrillseekerXTC mkIII | `ThrillseekerXTCmkIII (64).dll` |
| FerricTDS mkII | `FerricTDSmkII (64).dll` |
| Frontier | `Frontier.dll` |
| Dr Phase | `Dr Phase.dll` |
| IVGI2 | `IVGI2.dll` |
| BBE Sonic Sweet | `Harmonic Maximizer.dll`, `Mach 3 Bass.dll`, `Loudness Maximizer.dll`, `Sonic Maximizer.dll` |
| LoudMax | `LoudMax64.dll` |
| PhaseNudge | `PhaseNudge64.dll` |
| T-De-Esser | `T-De-Esser.dll`, `T-De-Esser.vst3` |
| TDR Nova / Molotok / VOS SlickEQ | `TDR *.dll` |
| XBass4000L | `XBass4000Lx64.dll` |

Copie additionnelle T-De-Esser : `C:\Program Files\VSTPlugins\T-De-Esser.dll`

---

## Interaction Soundpad

> **État local (2026-07-14) :** Soundpad **non lancé**, UniteFx **inactif** — confirmé par l'utilisateur.  
> L'audit initial listait la DLL UniteFx sur le device ; elle peut rester installée sans être active.

Si UniteFx est réactivé, le micro fifine aurait **deux APO** en série :

1. **UniteFx** (Soundpad) — injection soundboard
2. **EqualizerAPO** — chaîne VST ci-dessus

Dans ce cas, toute lecture Soundpad traverserait aussi les 8 VST. **Avec MixMixer, laisser UniteFx désactivé** pour éviter double injection.
