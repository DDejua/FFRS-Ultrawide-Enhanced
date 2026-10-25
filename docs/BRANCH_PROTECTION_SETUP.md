# Enable mandatory GitHub branch protection — FFRS

> **Action required by the repository administrator.** The currently connected GitHub application can edit repository files and PRs but **does not have GitHub Administration permission to create Rulesets or branch protection settings**. The automation below is committed to this repository; the ruleset must be enabled once in the GitHub UI to make the checks mandatory.

## Goal: fixes accumulate, never replace

- `main` = stable code currently published on Nexus.
- `develop` = union of all accepted fixes for the next release.
- Each experimental fix uses its own branch from the **latest `develop`** and goes through a PR targeting **`develop`**.
- New work must include all commits already in the target branch. If `develop` moved since branching, update the feature branch using a merge or rebase **without dropping the feature's own commits**.
- Once all fixes are integrated and tested **together** from `develop`, a single `develop` → `main` PR creates the next release.
- **Do not force-push, reset, overwrite branches or copy an old test ZIP as the release binary.**

## 1. Create the ruleset (main and develop)

Open [repository Settings → Rules → Rulesets](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/settings/rules) using the account that owns the repository.

1. Select **New ruleset → New branch ruleset**.
2. **Ruleset name:** `FFRS — protect main and develop`.
3. **Enforcement status:** `Active`.
4. Under **Target branches**, choose **Include by pattern** and add **`main`** and **`develop`** (as two exact patterns; do not inadvertently target `test/*`).
5. Enable **Restrict deletions** and **Block force pushes**.
6. Enable **Require a pull request before merging**. On a single-maintainer repository, leave required review approvals at **0** unless you have another reviewer available: authors cannot approve their own PRs.
7. Enable **Require status checks to pass** and add both:
   - `verify-cumulative-pr` from **Integration guard**
   - `windows-build` from **Build Windows candidate / PR validation (not a release)**
   
   Choose the status entries shown by GitHub after a PR has run; their fully displayed names may include the workflow prefix.
8. If available, select **Require branches to be up to date before merging** (strict checks).
9. Review any **bypass list**. Remove unnecessary bypass permissions if the account supports it, otherwise even an owner could bypass checks.
10. Press **Create** / **Save changes** and confirm the ruleset is **Active**.

Some Github plan/repository configurations may restrict enforcement options. If a setting is not offered, do **not** assume the repo is protected: check GitHub's effective rule summary and report which fields are available.

## 2. Confirm protection works

Create a harmless docs-only PR into `develop`, then verify that **both** required checks appear and you cannot merge until they finish. Open a test PR with a runtime change targeting `main` from `test/*` and verify that `verify-cumulative-pr` refuses it. **Do not merge the intentionally failing PR.**

Only after the ruleset is active do we have server-side enforcement preventing direct pushes to `main` and `develop`. The workflow by itself runs on PRs but **does not stop direct pushes**.

## 3. What each required check verifies

### `verify-cumulative-pr`

- Checks that the PR head **contains** the target branch's previous commit history. An outdated branch fails instead of silently replacing previously integrated work.
- Rejects a runtime / CI PR into `main` unless its source is this repository's **`develop`** branch.
- Rejects accidental deletion of a fix module under `overrides/src/fixes/` during PR.
- Documents that logical regressions **cannot be excluded automatically**. A change to the contents of an existing Rust file can still break older code.

### `windows-build`

Builds from pinned Windows Rust source with license, loader and privacy audits. It runs on **all PRs** to `main` and `develop`, so its status is available as a mandatory check even for docs changes.

### Human test gate (cannot be proven by CI)

Before merging a functionality PR into `develop`, record user validation of that **exact ASI SHA / source commit** in [TEST_MATRIX.md](TEST_MATRIX.md) and the linked Issue/PR. Before merging `develop` into `main`, **compile and test the combined code** (castle, cinematics, DOF, menus and 3 HUD presets), then build the manual-only distribution ZIP and check hashes. Never claim a single-feature test covers a combined build.

## 4. Emergency fixes

If production has a severe regression, prepare a hotfix **from current `develop`**, test it, and release it through the same PR route. Don't overwrite `main` with a local binary or bypass protection just to speed up an update. If necessary, temporarily delay publication while maintaining an Issue explaining the risk.

## Canonical links

- [Project status](../PROJECT_STATUS.md)
- [Development workflow](DEVELOPMENT_WORKFLOW.md)
- [Game test matrix](TEST_MATRIX.md)
- [Next-release integration tracker](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/issues/7)
