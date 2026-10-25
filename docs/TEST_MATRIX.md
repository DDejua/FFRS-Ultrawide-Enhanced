# Game testing matrix and provenance

**Do not confuse compilation with successful gameplay.** Tests must name *the exact binary* (file SHA-256), source commit SHA and game setup. Data here is intentionally conservative.

## Release / historical baseline

| Candidate | Source / reference | Windows build | Game test | Decision |
| --- | --- | --- | --- | --- |
| Nexus public **v1.0.1** (manual-only) | [PR #1](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/pull/1), source `a2086e71a5a1e84f527f5f498cc1c655962db1ad` | [CI #40078489074](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/actions/runs/40078489074) | HYBRID menu fix (3440×1440) observed working; user later encountered one castle exploration crash. No verified post-upload complete regression result recorded. | **PUBLISHED with open crash issue #4; caution.** |
| Crash hypothesis — **pending-state guard** | [`test/encounter-pending-reset-v102`](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/tree/test/encounter-pending-reset-v102) | [CI #40168924552](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/actions/runs/40168924552) — success | **NOT CONFIRMED**; precise ASI not yet reported tested in game | **NOT MERGED / NOT PUBLISHED** |
| DOF observational repro / opt-in telemetry | [Issue #5](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/issues/5), `test/dof-diagnostics-issue5` | Unverified: see PR/CI | **Visual repro only:** 16:9, mod ON, depth blur visible; 3440×1440, mod ON, depth blur greatly reduced. Diagnostic ASI not yet tested. | **RESEARCH / SOURCE PROBE; NOT A FIX** |

> Earlier v1.0.0 testing confirmed multiple modes and installation behavior. It does not validate a subsequently rebuilt or combined binary.

## Required regression tests before the next release

| Case | HYBRID 3440×1440 | ULTRAWIDE 3440×1440 | CLASSIC 3440×1440 | Other 16:9 / 32:9 |
| --- | --- | --- | --- | --- |
| Game starts; loader and ASI load without warnings | ☐ | ☐ | ☐ | ☐ |
| Save load and extended castle exploration | ☐ | ☐ | ☐ | ☐ |
| Encounter start/end and repeated battle transitions | ☐ | ☐ | ☐ | ☐ |
| Cinematics/flashbacks and their aspect ratio | ☐ | ☐ | ☐ | ☐ |
| System menu Y / ESC (open/close repeatedly) | ☐ | ☐ | ☐ | ☐ |
| Mission menu J / RB (open/close repeatedly) | ☐ | ☐ | ☐ | ☐ |
| Minimap / quest HUD restore when player is motionless | ☐ | ☐ | ☐ | ☐ |
| Switching TOML presets / game restart | ☐ | ☐ | ☐ | ☐ |
| Manual install and manual uninstall / backups | ☐ | ☐ | ☐ | ☐ |

Do not check a box without an actual result. Document exceptions, untested combinations and hardware differences in Issue/PR.

## New test entry template

Copy the block below into a PR or Issue comment (or append an entry to this file):

```text
Candidate:
Issue / PR:
Source branch + exact SHA:
Windows CI run:
ASI SHA-256:
ZIP SHA-256 (if tested):
Game build/storefront:
GPU / driver (when relevant):
Resolution + HUD preset:
Test actions / duration:
PASS / FAIL / NOT TESTED:
Relevant sanitized log/exception:
Regressions observed:
Tester / date:
```

For crash testing, retain original .dmp privately, not in the public repository.
