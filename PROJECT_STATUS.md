# FFRS Ultrawide Fix + DOF — PROJECT STATUS

**Canonical status dashboard · Last reviewed: 2026-10-25 · Update this file whenever a feature branch is opened, a candidate is tested, or a release is shipped.**

The [GitHub repository](https://github.com/DDejua/FFRS-Ultrawide-Enhanced) is the single source of truth. **Chat history, local ZIP filenames and unpublished builds are not authoritative.** Every chat and contributor must read this dashboard and [the development workflow](docs/DEVELOPMENT_WORKFLOW.md) before modifying code or preparing a binary.

## Current release — v1.1.0 (2026-10-25)

**Nexus v1.1.0 published and author-tested**: [FFRS Ultrawide Fix + DOF, mod #71](https://www.nexusmods.com/finalfantasyresonance/mods/71?tab=files). The approved game ASI was compiled from source commit `0229fcb3d93fd6616a691c69666a7d8721b20ba5` and its SHA-256 is `7c9b091522ad8e4606895218f34690551254e0b5c13426abbfefd53a057015d1`. The exact manual player ZIP is `FFRS Ultrawide DOF Fix V1.1_RimuModding.zip`, SHA-256 `a3abfbe40a0e55002876fa988e67896ad1eccc8fc52adc4e69df546dcc2d26b0`. Source integration tracked in [PR #18](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/pull/18); release metadata in [RELEASE_NOTES_v1.1.0.md](RELEASE_NOTES_v1.1.0.md).

- **DOF:** enabled by default at ultrawide resolutions; mathematically exact two-anchor fit 16:9 -> 1.00 and 3440×1440 -> 1.50, with bounded theoretical predictions at other ratios. 32:9/higher remain **unverified**.
- **Crash fix:** incorporates the exact encounter no-crop pending-state reset source `fafe80a7e8fdd012bf4a28a924c1cb5dc841c804`; several hours without the previously reported crash during author testing. Not a universal stability guarantee.
- **HYBRID, ULTRAWIDE and CLASSIC** HUD presets remain; pre-rendered 16:9 FMVs preserved. HYBRID default.
- **Install:** manual player ZIP, official pinned/hash-verified `winmm.dll` plus ASI/TOML; no `.cmd`, `.bat` or `.ps1` executable installers. Borderless Windowed is recommended to avoid Fullscreen stretching.
- **Opt out:** `[post_processing] ultrawide_dof_fix = false` keeps native DOF; `depth_of_field = false` disables all DOF.
- **Nexus, Steam and modders Discord announcements** reported complete by the author. GitHub branch/PR/release state must be verified independently and never inferred from those announcements.

**Source vs distribution:** the release archive includes the exact author-approved game binary. GitHub source documentation updates may add commits without modifying that compiled binary; only the v1.1.0 source tree (not later docs) should be used to reproduce the released ASI fingerprint.

## Branches — their purpose

| Branch | Meaning |
| --- | --- |
| `main` | **Public, released stable source**. Merge only once combined changes are validated and a release candidate approved. Docs-only updates are fine. |
| `develop` | **Next release integration.** Only tested feature fixes are merged here via pull requests. It may contain unreleased work and must not be distributed as stable. |
| `test/*`, `fix/*`, `feature/*` | Isolated experiments, **branch from `develop`**. One issue/workstream per branch. CI passing != gameplay verified. |
| `chore/*`, `docs/*` | Repository organization and documentation changes, via PR. |

Existing pre-workflow branches were created off `main` before the `develop` policy; they remain historical experiments. Do not merge them wholesale. Prefer **cherry-picking/reapplying the specific verified change to a fresh branch based on `develop`**, then create a PR.

## Release tracking and follow-ups

**Published feature integration:** [PR #18](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/pull/18) addresses [Issue #4 (encounter stability)](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/issues/4) and [Issue #5 (DOF)](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/issues/5). [Issue #7](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/issues/7) tracks cumulative release gates and notes. The user has approved the combined v1.1 build and uploaded it on Nexus.

**Future testing:** Find a 32:9 monitor owner for 5120×1440 or larger DOF calibration; investigate any report of incorrect resolution/fullscreen stretching; preserve the previous Issue #4 crash fix in all future updates. Gamepad Y-menu behavior was observed in the unmodded game and is not fixed here.

## Branch protection enforcement status

- [x] `Integration guard` workflow prepared: PR ancestry, `develop` → `main` release gate and fix-module deletion detection.
- [x] Windows build configured to run on all PRs into `main` or `develop` for mandatory CI checks.
- [x] **OWNER ACTIVATED RULESET on 2026-10-25:** [FFRS - Protect main and develop (ID 25813463)](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/settings/rules/25813463) was independently verified via GitHub Rulesets REST GET. `enforcement=active`; target refs exactly `main` and `develop`; empty bypass list; required PR with 0 reviewers; deletion/force-push restrictions; mandatory GitHub Actions checks `verify-cumulative-pr` and `windows-build`; strict up-to-date policy.
- [x] GitHub branch GET reports `protected=true` for both `main` and `develop`. The generic branch-protection subfield is not authoritative for Rulesets; the Ruleset endpoint confirms enforcement.
- [ ] **Optional adversarial confirmation:** create a deliberately stale PR to validate it is blocked by `verify-cumulative-pr` (never merge this test). No direct-push or force-push attempt has been performed, to avoid altering stable history.
- [ ] No automatic guarantee of bug-free cumulative *behavior*: changes to existing code can still regress older fixes, so the combined release must be tested in-game.

## Hard requirements for a patch to count as integrated

1. A **GitHub Issue** explains the bug/feature, observations, reproduction, evidence and acceptance criteria.
2. A **separate branch from `develop`** contains code changes; a PR documents exact changes.
3. CI builds the **specific commit SHA** and preserves build run + artifact reference.
4. Game tests are recorded with tested binary/source SHA, mode, resolution, steps, result and log. Use [TEST_MATRIX](docs/TEST_MATRIX.md).
5. PR is merged into `develop` only after user validation. All pending approved patches must coexist in the same `develop`.
6. A **combined integration candidate** built from the current `develop` is tested. If it regresses, fix on `develop` without removing unrelated features.
7. Only a release PR from `develop` to `main` may advance production. Use an explicitly named release version, an SHA-256-verified **manual-install-only** ZIP, notes and final tests. Upload to existing Nexus page only when approved.
8. Update this dashboard and relevant issue/PR whenever the state changes. Avoid undocumented changes or anonymous local binaries.

## Cross-chat start-of-session checklist

> **Always begin by reading `PROJECT_STATUS.md`, `docs/DEVELOPMENT_WORKFLOW.md`, currently open GitHub Issues and PRs, and inspecting the tip of `main` and `develop`.**
> Choose an Issue, branch from `develop`, keep the user-test build on that branch, and never assume an artifact in a different chat contains the current changes. Before creating a ZIP, verify the source commit and diff to `develop`.

This repository tracks the truth; a conversation can suggest code, but **a fix is not part of a release until it has a recorded PR merge and matching tested build**.
