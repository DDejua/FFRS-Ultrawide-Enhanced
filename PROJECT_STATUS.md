# FFRS Ultrawide Hybrid Fix — PROJECT STATUS

**Canonical status dashboard · Last reviewed: 2026-10-25 · Update this file whenever a feature branch is opened, a candidate is tested, or a release is shipped.**

The [GitHub repository](https://github.com/DDejua/FFRS-Ultrawide-Enhanced) is the single source of truth. **Chat history, local ZIP filenames and unpublished builds are not authoritative.** Every chat and contributor must read this dashboard and [the development workflow](docs/DEVELOPMENT_WORKFLOW.md) before modifying code or preparing a binary.

## Current release — stable

| Item | State |
| --- | --- |
| Released version | **v1.0.1** (manual install only) |
| Nexus | [FFRS Ultrawide Hybrid Fix, mod #71](https://www.nexusmods.com/finalfantasyresonance/mods/71?tab=files), publicly downloadable / scanner marked safe as reported by uploader on 2026-10-24 |
| Canonical released source | `main`: v1.0.1 functionality; newer documentation-only commits do not change the shipped binary |
| Latest code change on `main` for v1.0.1 | [PR #1](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/pull/1), commit `a2086e71a5a1e84f527f5f498cc1c655962db1ad` (source was then edited only in documentation) |
| Runtime validated | 3440×1440 Windows Steam, three HUD modes during development; game-thread minimap/quest menu restoration user-tested |
| Known issue | Controller Y sometimes fails to reopen menu; reproduced unmodded, cause/frequency difference not determined |
| Final upload package | Script-free ZIP; `README.txt`, three HUD presets, loader, ASI and license notices; no `.cmd` or `.ps1` |
| Published ASI fingerprint | Preserve/confirm the published binary hash when preparing the next release; a passing CI build alone is not evidence that a ZIP is identical |

**NEVER overwrite `main` with a test branch or replace the released Nexus ZIP because a test compiled.** A release needs its own integration and in-game verification. Do not treat a GitHub Actions artifact as the final mod package.

## Branches — their purpose

| Branch | Meaning |
| --- | --- |
| `main` | **Public, released stable source**. Merge only once combined changes are validated and a release candidate approved. Docs-only updates are fine. |
| `develop` | **Next release integration.** Only tested feature fixes are merged here via pull requests. It may contain unreleased work and must not be distributed as stable. |
| `test/*`, `fix/*`, `feature/*` | Isolated experiments, **branch from `develop`**. One issue/workstream per branch. CI passing != gameplay verified. |
| `chore/*`, `docs/*` | Repository organization and documentation changes, via PR. |

Existing pre-workflow branches were created off `main` before the `develop` policy; they remain historical experiments. Do not merge them wholesale. Prefer **cherry-picking/reapplying the specific verified change to a fresh branch based on `develop`**, then create a PR.

## Workstreams in progress

**Next combined release tracker:** [Issue #7 — integration and publication gates](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/issues/7). This is the checklist for combining independently tested branches into one release.

| Workstream | Tracking | Branch / build | State | Next evidence |
| --- | --- | --- | --- | --- |
| Castle exploration crash, memory access violation `0xC0000005` | [Issue #4](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/issues/4) | [`test/encounter-pending-reset-v102`](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/tree/test/encounter-pending-reset-v102), [Windows CI #40168924552](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/actions/runs/40168924552) | **COMPILED, GAME TEST NOT CONFIRMED. NOT MERGED / NOT RELEASED.** | Check encounter/cinematic behavior and extended castle exploration. If passed, rebase/reapply patch onto fresh `develop` branch and PR. |
| Depth of field / ultrawide postprocessing | [Issue #5](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/issues/5) | No code branch or candidate confirmed | **RESEARCH ONLY; NO PATCH YET** | Reproduce and characterize DOF before proposing fix. |
| Input Y requiring repeated press | Tracked as known limitation in README; may become issue if actionable | No active fix | **UNMODDED REPRODUCTION CONFIRMED; NOT SOLVED** | Record a consistent repro and isolate native vs modded frequency if needed. |

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
