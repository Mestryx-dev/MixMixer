## Summary

<!-- What does this PR change and why? -->

## Type of change

- [ ] Bug fix
- [ ] New feature
- [ ] Documentation
- [ ] Refactor / chore

## Test plan

<!-- How did you verify audio routing and UI behavior? -->

- [ ] Built with `cargo build --release`
- [ ] Tested mic → CABLE Input → CABLE Output → Discord (or equivalent)
- [ ] Settings live apply (routing, devices, listen volume) and tray restore work
- [ ] No unintended changes to `config.json` committed

## Checklist

- [ ] CHANGELOG.md updated (if user-facing)
- [ ] New UI strings added to `src/i18n/` (not hardcoded)
- [ ] `cargo fmt` and `cargo clippy` pass locally
