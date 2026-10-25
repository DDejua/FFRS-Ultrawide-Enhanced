## Linked workstream
- Issue: #
- Target: `develop` (feature fix) / `main` (validated release) / docs
- Base branch checked just before PR:

## Changes
- What changed:
- What this does **not** claim or fix:
- Source commit SHA and risk:

## Build provenance
- CI run:
- Binary SHA-256:
- ZIP SHA-256 (if packaged):

## Evidence / tests
- [ ] Windows CI build passes for the exact PR SHA.
- [ ] User test performed on the exact candidate ASI; results recorded below.
- [ ] Castle exploration + multiple battle transitions.
- [ ] Menu Y/ESC, J/RB; minimap and quest HUD restore when idle.
- [ ] Three HUD presets and resolution compatibility, or explicitly note what's untested.
- [ ] No unrelated workstream changes removed.
- [ ] Update `PROJECT_STATUS.md` and `docs/TEST_MATRIX.md` where applicable.

### Exact test environment and results
- Game version / platform:
- Resolution / preset:
- Test duration / steps:
- PASS / FAIL / NOT TESTED:
- Sanitized logs and crash info (never raw public .dmp):

## Merge and publication gate
- [ ] If feature: merges into **`develop` only** after user acceptance.
- [ ] If release: **combined `develop`** tested, then PR to `main`.
- [ ] For Nexus upload, the final tested ZIP is **manual-only**, SHA-256 verified, and has licenses.
- [ ] No public release without explicit approval.
