# Changelog

All notable changes to MixMixer are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-07-15

### Added

- Centralized i18n module (`mix-mixer/src/i18n/`) with English (default) and French UI strings
- `locale` config field and `MIXMIXER_LANG` environment override
- Root README, tutorial, issue templates, and contributing guide
- MIT license

### Changed

- Settings window height computed to fit content exactly (no scroll, no empty space)
- Public documentation and code comments in English
- `config.json` excluded from version control (use `config.example.json`)

### Fixed

- Apply button no longer closes the settings window
- Auto-reconnect when Windows audio devices change
- Release builds no longer show a console window on startup

## [0.1.0] - 2026-07-14

### Added

- Initial release: microphone → VB-Cable routing with WASAPI
- System tray menu and egui settings window
- Optional headphone monitor bus
- Device substring matching and `--list-devices` CLI
- Live latency and buffer metrics in settings UI

[0.1.1]: https://github.com/Mestryx-dev/MixMixer/releases/tag/v0.1.1
[0.1.0]: https://github.com/Mestryx-dev/MixMixer/releases/tag/v0.1.0
