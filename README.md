# FINAL FANTASY RESONANCE — Ultrawide Enhanced (community source overlay)

**Source review package — not yet a public release.**

This project provides **only the changed source files**, relative to the upstream
[PolarWizard FINAL FANTASY RESONANCE Fix](https://github.com/PolarWizard/final-fantasy-resonance-fix)
version **0.3.0** (`0f5bc2c4bab2b1680c59d8d1c2135253ae14c0df`).
The upstream repository provides the full Rust application and the Unreal Engine Rust SDK.
No game files, SDK dump, personal builds, or local user profile paths are included here.

## What is changed?

- Independent minimap relocation for the `WBP_MiniMapLayout_C` root; offset is
  derived from actual viewport geometry and DPI. Added offsets are 18 px right,
  18 px up on wider-than-16:9 viewports, and zero for 16:9.
- TOML `[minimap]` options `enable`, `probe`, and `name_hint`.
- Battle transition scratch render-target lifetime correction from contributor
  [Darjjj's upstream PR #3](https://github.com/PolarWizard/final-fantasy-resonance-fix/pull/3)
  (the PR is third-party work, not independently invented by this project).
- The base PolarWizard 16:9/21:9/32:9 HUD selection remains intact. Our end-user
  package offers ULTRAWIDE (21:9 HUD), CLASSIC (centered 16:9 HUD), and HYBRID
  (centered 16:9 HUD plus independently positioned minimap).

## Windows build — no local end-user compiler needed

1. Install Git, Rust 1.99.1, and the Visual C++ MSVC build tools *if building locally*.
2. Run `scripts\prepare-upstream.ps1` from PowerShell. It clones the exact
   upstream version and overlays the modified files into `upstream/`.
3. `cd upstream` and run `cargo build --locked --release`.
4. `target\release\final_fantasy_resonance_fix.dll` can be renamed to
   `final_fantasy_resonance_fix.asi` when validated.

For a **cloud build**, GitHub Actions > **Build Windows candidate** > Run workflow.
It uses Windows and Rust 1.99.1 and exports a **test artifact, not a release**.

The workflow tries to remap local source paths and refuses to export an ASI
if an unremapped Windows user directory pattern is still detected.
A successful build or path scan does **not** guarantee functionality, privacy,
or binary reproducibility. Test against a clean Steam install before release.

## Testing and known limits

- Windows Steam, 3440 × 1440: three HUD modes and several hours of battle
  transition testing were reported successful by a tester on 2026-10-24.
- 16:9 streaming: reported working on the same machine.
- 2560 × 1080, 3840 × 1600, 32:9 and other game builds are **not yet certified**.
- The ultrawide root code, not just our minimap, may still install hooks on
  native 16:9. Do not claim 'no patches at all at 16:9'.
- Changing mode requires game restart; the public scripts and preset TOML files
  are distributed with the **end-user** archive, not this source overlay.

## Licensing / credits

- **PolarWizard / Dominik Protasewicz**: upstream game fix, MIT. Retain full notice.
- **Darjjj**: encounter scratch-target lifetime correction from upstream PR #3.
- **ThirteenAG**: Ultimate ASI Loader, MIT, bundled separately with the end-user build.
- **Minimap extension, HUD selector, packaging**: community modifications.

`LICENSES/` contains copies of the upstream notices. Review the Rust dependency
license inventory and bundle any required third-party notices before publication.
See `RELEASE_GATES.md` and `THIRD_PARTY_DEPENDENCIES.md`.

This project is unofficial and is not affiliated with the game's developers,
publishers or original mod authors.
