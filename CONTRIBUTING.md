# Contributing to MixMixer

Thank you for your interest in contributing. MixMixer is a Windows audio routing tool built in Rust.

## Getting started

1. Fork the repository and clone it locally.
2. Install [Rust](https://rustup.rs/) and MSVC Build Tools on Windows.
3. Install [VB-Audio Virtual Cable](https://vb-audio.com/Cable/) for manual testing.
4. Build and run:

   ```powershell
   cd mix-mixer
   copy config.example.json config.json
   cargo run -- --list-devices
   cargo run
   ```

## Project layout

| Path | Purpose |
|------|---------|
| `mix-mixer/src/audio/` | WASAPI engine, mixer, resampling |
| `mix-mixer/src/ui/` | egui settings window, tray icon |
| `mix-mixer/src/i18n/` | UI strings and locale resolution |
| `docs/` | Internal specs and validation notes |

## Guidelines

- **Scope**: Keep pull requests focused on one change.
- **Language**: User-facing docs and new code comments in **English**. UI strings go in `src/i18n/`.
- **Config**: Never commit `config.json` — only `config.example.json`.
- **Style**: Match existing Rust patterns; run `cargo fmt` and `cargo clippy` before submitting.
- **Testing**: Describe manual audio tests in the PR (Discord + VB-Cable chain).

## Adding a language

1. Add `mix-mixer/src/i18n/<code>.rs` with a `TEXTS` constant.
2. Extend `Locale` in `mix-mixer/src/i18n/mod.rs`.
3. Document the locale code in README and `config.example.json`.

## Reporting issues

Use the [bug report](https://github.com/Mestryx-dev/MixMixer/issues/new?template=bug_report.yml) or [feature request](https://github.com/Mestryx-dev/MixMixer/issues/new?template=feature_request.yml) templates.

Include Windows version, device names, and steps to reproduce audio issues.

## Pull requests

1. Update `CHANGELOG.md` under `[Unreleased]` or the next version section.
2. Fill in the PR template checklist.
3. Link related issues when applicable.

Questions? Open a [Discussion](https://github.com/Mestryx-dev/MixMixer/discussions) or an issue.
