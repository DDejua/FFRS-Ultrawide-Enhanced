---
name: Bug report / investigation
about: Track an FFRS bug with reproducible evidence and isolated test builds
title: "Bug: "
labels: []
assignees: []
---

## Symptom and expected behavior
**Actual:**
**Expected:**

## Reproduction and environment
- Game version / platform:
- Windows / GPU / driver:
- Resolution, display mode and HUD preset:
- Mod version and `.asi` SHA-256:
- Steps to reproduce:
- Occurs without mod? / untested:

## Evidence
- Sanitized game/mod logs and event IDs (redact usernames and paths):
- Exception code / module offset / WinDbg stack when available:
- Screenshots or video:

**Do not upload raw memory dumps to public issues**; they can contain personal data.

## Hypothesis (not confirmed diagnosis)
-

## Proposed experiment
- Branch from `develop`: `test/...`
- Windows CI run / exact SHA:
- Expected effect:

## Acceptance criteria
- [ ] Reproduction reduced or eliminated, with test details
- [ ] No regression in main HUD modes, encounter transitions and menus
- [ ] User test logged in `docs/TEST_MATRIX.md`
- [ ] PR into `develop` only after confirmation
