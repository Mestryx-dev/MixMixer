# MixMixer — Setup tutorial

Step-by-step guide to route your processed microphone through VB-Cable to Discord, games, and OBS.

## What you will achieve

```
Your mic (with E-APO / VST)  →  MixMixer  →  CABLE Input  →  CABLE Output  →  Discord / GTA / OBS
```

MixMixer does **not** replace your soundboard. Other apps can still send audio to **CABLE Input**; Windows mixes them with your voice.

## Prerequisites

1. **Windows 10 or 11** (64-bit)
2. **[VB-Audio Virtual Cable](https://vb-audio.com/Cable/)** — download, run installer as administrator, reboot if prompted
3. **(Recommended)** [Equalizer APO](https://sourceforge.net/projects/equalizerapo/) on your physical microphone
4. **MixMixer binary** — build from source or download from [Releases](https://github.com/Mestryx-dev/MixMixer/releases)

## Step 1 — Install VB-Cable

After installation you should see in Windows Sound settings:

- **CABLE Input** — playback device (MixMixer writes here)
- **CABLE Output** — recording device (Discord reads from here)

If they are missing, reinstall VB-Cable as administrator.

## Step 2 — Configure Equalizer APO (optional)

1. Open the E-APO configurator.
2. Attach Equalizer APO to your **physical microphone** (e.g. USB mic).
3. Apply your VST chain (noise gate, EQ, etc.) on that device.

MixMixer captures **after** E-APO processing on the same device name.

## Step 3 — First launch (config is automatic)

Just run `mix-mixer.exe`. On first start MixMixer creates:

```
%APPDATA%\MixMixer\config.json
```

Defaults are filled from your PC:

- **Microphone** → Windows default input device  
- **Virtual mic** → `CABLE Input` when VB-Cable is installed  
- **Monitor** → Windows default playback (non-cable)  
- **Language** → system UI language when `en` / `fr` is detected  

Open settings and adjust devices if needed. List all devices:

```powershell
mix-mixer.exe --list-devices
mix-mixer.exe --print-config-path
```

Device names are **substring matches** — `"fifine"` works if only one device contains that text.

## Step 4 — Set Discord / game microphone

In Discord → **User Settings → Voice & Video → Input Device**:

- Select **CABLE Output** (not your physical mic)

Same for GTA voice chat, OBS audio input, etc.

## Step 5 — Launch MixMixer

```powershell
.\mix-mixer.exe
```

- Settings window opens automatically.
- Tray icon appears; **left-click or double-click** to reopen settings.
- Header shows **Active**, latency in ms, and buffer fill when routing works.
- **FR** / **EN** chips under the metrics switch the UI language.
- Closing the window (×) or minimizing **hides to tray** — MixMixer keeps running.
- Clicking the minimized app on the **taskbar** also restores the settings window.

Click **Apply** after changing devices or listen volume. A short toast confirms; the window stays open.

**Écoute casque / Listen volume** only changes how loud you hear yourself in headphones — Discord / CABLE stays at unity.

## Step 6 — Verify audio

1. Speak into your mic — Discord input meter should move.
2. Confirm Discord still uses **CABLE Output**.
3. Optional: enable **Headphone monitor** to hear yourself on your headphones.

## Using a soundboard

Configure your soundboard app to output to **CABLE Input**. Windows mixes soundboard + MixMixer voice on the same virtual cable. Discord hears both on **CABLE Output**.

## System tray

| Action | Effect |
|--------|--------|
| Left-click or double-click tray icon | Open / restore settings |
| Click taskbar window icon (when minimized) | Restore settings |
| Right-click → About | Show about dialog (version + project link) |
| Right-click → Quit | Exit MixMixer |

To quit, use the tray **Quit** item — closing the settings window only hides it.

## Tuning latency

| `buffer_frames` | Approx. latency @ 48 kHz |
|-----------------|--------------------------|
| 64 | ~1.3 ms (may crackle on slow CPUs) |
| 128 | ~2.7 ms (default) |
| 256 | ~5.3 ms |
| 512 | ~10.7 ms (stable) |

If you hear crackling, increase `buffer_frames`.

## Changing language

Click **FR** or **EN** in the settings header (saved to `config.json`), or set:

```json
"locale": "fr"
```

Or:

```powershell
$env:MIXMIXER_LANG = "fr"
.\mix-mixer.exe
```

## Common mistakes

| Mistake | Result |
|---------|--------|
| Discord mic = physical device | E-APO chain bypassed or double routing |
| Discord mic = CABLE Input | Wrong endpoint — use **CABLE Output** |
| MixMixer not running | CABLE Output is silent |
| Wrong substring in config | Wrong device selected — run `--list-devices` |

## Uninstall / reset

1. Quit MixMixer from the tray menu (right-click → Quit).
2. Delete `%APPDATA%\MixMixer\config.json` to reset to auto-defaults on next launch.
3. VB-Cable can stay installed for other apps.

## Next steps

- [README](../README.md) — overview and config reference
- [CONTRIBUTING.md](../CONTRIBUTING.md) — development setup
- [GitHub Issues](https://github.com/Mestryx-dev/MixMixer/issues) — report bugs
