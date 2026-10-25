# Multi-chat / multi-workstream development workflow

This is the repeatable process for any contributor or ChatGPT conversation working on FFRS. **Use GitHub state, not ChatGPT memory, as the authority.** To make this process mandatory at server level, the repository owner must first [enable the branch protection ruleset](BRANCH_PROTECTION_SETUP.md). GitHub workflows alone do not block direct pushes.

## 1. Determine the baselines

- `main`: the current released/stable code for Nexus (never use for speculative edits).
- `develop`: next-version integration baseline. It is **not public release**.
- Check [PROJECT_STATUS.md](../PROJECT_STATUS.md) and [TEST_MATRIX.md](TEST_MATRIX.md), plus Issues and open PRs.
- Confirm the exact `main` and `develop` commit SHA before editing. If a separate chat changed `develop` while work was ongoing, update your feature branch or apply your verified patch again to a fresh branch.

## 2. Isolate every experiment

- Create or update one GitHub Issue for each independently testable problem (crash, DOF, minimap, 32:9, etc.).
- Create `test/<topic>` or `fix/<issue>-<topic>` from the **latest `develop`**, not from `main` or another test branch.
- A candidate ZIP must include a README/manifest stating issue ID, branch, full source commit SHA, build run and SHA-256. The CI artifact alone is **not** the final Nexus ZIP.
- Do not silently swap the `.asi` between unrelated candidates. **Only the candidate built from the exact stated commit may be called tested.**

## 3. Record the tests before merging

For each candidate record at minimum:

| Field | Example |
| --- | --- |
| Issue | `#4` |
| Branch / commit | `fix/4-encounter-guard`, SHA |
| CI | Link to GitHub Actions run |
| Files tested | ASI SHA-256 / optional ZIP SHA-256 |
| Environment | Windows Steam; 3440×1440; HYBRID |
| Test actions | Castle exploration, multiple encounters, menu rapid open/close |
| Result | PASS / FAIL / NOT TESTED; links to logs/dumps stored **privately** |
| Regressions | CLASSIC, ULTRAWIDE, 16:9 and changed settings |

Link records in Issue comments / PR body; update [TEST_MATRIX.md](TEST_MATRIX.md). **Never upload raw Windows crash dumps publicly**: they may contain private process data. Include only sanitized exception IDs, module offsets, stack excerpt and logs with personal paths redacted.

A successful build is **BUILD OK**, not **GAME PASS**. A plausible fix is **HYPOTHESIS**, not **FIX CONFIRMED**.

## 4. Integrate in order, never overwrite

1. Open feature PR targeting `develop`, not `main`.
2. Review diff and check for overlap/conflicts with other workstreams. If there are updates on `develop` since branching, update/recreate from latest `develop` before merging.
3. CI must pass on the current PR commit. User must confirm gameplay test before merge.
4. Merge feature PR into `develop`. Never copy a local ZIP or replace `main` with a feature branch.
5. After any merge, produce a **new combined `develop` build**. Earlier isolated-feature ZIPs are stale as soon as `develop` has gained other changes.
6. For the next release, open `develop` → `main` PR and run the complete regression matrix **against the combined source**; only then merge and publish.
7. Create the public **manual-only** ZIP from the tested build, preserve licensing and official loader, verify hashes and ensure no auto-install `.cmd`/`.ps1`. This public ZIP should have a tracked digest in the release notes.
8. Update `PROJECT_STATUS.md`, CHANGELOG, issue states, and Nexus release notes; test the ZIP as distributed.

## 5. Existing legacy test branches

The menu diagnostics and minimap prototypes under `test/menu-*` are archived experimental history. They are not release baselines. The currently known crash investigation branch `test/encounter-pending-reset-v102` was forked from `main` before this policy and has **not been user-verified**; port only its minimal fix to a fresh `develop` branch once justified.

## 6. How to ask a new chat to continue

Paste or say:

> Continue FFRS development from GitHub `DDejua/FFRS-Ultrawide-Enhanced`. Read `PROJECT_STATUS.md`, `docs/DEVELOPMENT_WORKFLOW.md`, `docs/TEST_MATRIX.md`, open Issues/PRs, and the heads of `main` and `develop`. Treat `main` as released; branch each experiment from `develop`. Do not merge tests, ship a binary or replace Nexus files without the validation gates. My workstream is Issue #X.

You should not have to paste old chat histories again. **If recorded GitHub state disagrees with a conversational summary, confirm the code/commits and amend the status file.**
