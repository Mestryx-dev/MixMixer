# MixMixer v0.1.1

Tweak release: professional repo setup, i18n, and settings window polish.

## Highlights

- **i18n**: English default UI; French via `"locale": "fr"` or `MIXMIXER_LANG=fr`
- **Settings UI**: Fixed window height — content fits without scroll or empty space
- **Stability**: Auto-reconnect on device change; Apply no longer closes settings
- **Docs**: README, tutorial, issue templates, MIT license

## Install

1. Download `mix-mixer.exe` from Assets (or build from source).
2. Copy `config.example.json` → `config.json` beside the exe.
3. Run `mix-mixer.exe --list-devices` and edit device names.
4. Set Discord microphone to **CABLE Output**.

See [docs/TUTORIAL.md](docs/TUTORIAL.md) for the full walkthrough.

## Upgrade from v0.1.0

- No breaking config changes. Add optional `"locale": "en"` if desired.
- Remove tracked `config.json` from your fork — use local copy only.

## Full changelog

[CHANGELOG.md](CHANGELOG.md)
