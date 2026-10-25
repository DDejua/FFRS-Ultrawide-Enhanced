# FINAL FANTASY RESONANCE — Ultrawide Fix + DOF

**v1.1.0 was published on Nexus Mods on 2026-10-25.** This repository contains the versioned source overlay and Windows build workflow for the mod.

**[Download v1.1.0 on Nexus Mods](https://www.nexusmods.com/finalfantasyresonance/mods/71?tab=files)** · [Manual installation instructions](INSTALLATION.md) · [Changelog](CHANGELOG.md)

### Development status across multiple conversations

**[PROJECT STATUS — read this first](PROJECT_STATUS.md)** · [Branching and integration workflow](docs/DEVELOPMENT_WORKFLOW.md) · **[Active branch protection ruleset](docs/BRANCH_PROTECTION_SETUP.md)** · [Test matrix and binary provenance](docs/TEST_MATRIX.md) · [Open Issues](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/issues) · [Pull requests](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/pulls)

Stable released source is `main`; new work is integrated through `develop` with review and game tests. Issues and release PRs record provenance; CI passing alone is not sufficient to publish an untested gameplay build.

The **published player-facing ZIP is manual-install only**: it contains `winmm.dll`, `scripts/final_fantasy_resonance_fix.asi`, the default `scripts/final_fantasy_resonance_fix.toml`, three `HUD_PRESETS`, `README.txt`, license notices, and support hashes/build information. **It contains no `.cmd` or `.ps1` installers.** No Vortex or automatic installation is claimed.

> **Source vs download:** PowerShell files in this *source repository's* `scripts/` directory prepare builds and run security/license audits. They are **developer build tools only** and are **not included in the public Nexus archive**, which installs by manual file copying.

## What's new in v1.1.0 — automatic DOF + stability

The combined source from [PR #18](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/pull/18) implements the encounter capture pending-state correction and a default-enabled resolution-aware DOF sensor compensation. The author tested and approved the exact public ASI at 3440×1440. [Optical model and limitations](docs/DOF_AUTO_FORMULA.md) · [Version changelog](CHANGELOG.md).

- `[post_processing] depth_of_field=true` retains DOF (default).
- `[post_processing] ultrawide_dof_fix=true` enables aspect-ratio-aware compensation (default).
- `depth_of_field=false` removes DOF completely, including ultrawide compensation.
- `ultrawide_dof_fix=false` disables *only* extra compensation, retaining original game DOF.
- User-tested at 3440×1440 (1.50×) including map, town and interiors; **32:9 and larger predictions are currently theoretical**.
- The published player archive is **manual copy only**, with zero `.cmd`, `.ps1` or `.bat` installers. The *source* repository's PowerShell build scripts are developer-only.

## HUD modes

- **HYBRID (default):** centered 16:9 HUD with the minimap positioned toward the right edge of an ultrawide display.
- **ULTRAWIDE:** 21:9 HUD.
- **CLASSIC:** centered 16:9 HUD.

The selected preset is a TOML configuration. Close the game and copy a different `HUD_PRESETS/<MODE>/final_fantasy_resonance_fix.toml` into `FFRS/Binaries/Win64/scripts/` to switch modes. **The DLL and ASI do not change between presets.** A 32:9 HUD root can be selected in TOML with `[hud] constrain = 2`, but 32:9 has not been independently certified.

## Changes and fix provenance

- Extends the 0.3.0 [PolarWizard FINAL FANTASY RESONANCE Fix](https://github.com/PolarWizard/final-fantasy-resonance-fix), pinned to `0f5bc2c4bab2b1680c59d8d1c2135253ae14c0df`, with resolution-aware independent positioning of `WBP_MiniMapLayout_C`.
- **v1.1.0:** adds default-on automatic DOF compensation and consumes the encounter pending state on an already-matching transition capture, addressing an intermittent crash. The 21:9 DOF behavior was compared to 16:9 and tested in the map, town and interiors. Larger formats need community validation.
- **v1.0.1 menu fix:** hides the independently positioned minimap and field quest/objective widget while centered system/mission menus are displayed. Original positions are preserved instead of visibly teleporting the minimap. Restores their opacity after approximately 160 ms, via a native game-thread frame callback even while idle, and cancels restoration if another menu opens.
- Preserves the original PolarWizard 16:9, 21:9 and 32:9 HUD root settings and related cinematic/UI corrections.
- Includes the battle transition scratch render-target lifetime correction from [Darjjj's upstream PR #3](https://github.com/PolarWizard/final-fantasy-resonance-fix/pull/3); this is credited third-party work.

## Install and uninstall

See [INSTALLATION.md](INSTALLATION.md) for a visual folder tree, backup precautions, switching HUD presets and safe manual removal.

In brief, after closing the game: copy `winmm.dll` beside `FFRS-Win64-Shipping.exe` in `FFRS/Binaries/Win64/`, and copy the two `scripts/` files into `FFRS/Binaries/Win64/scripts/`. Back up existing files and do not overwrite another mod's ASI loader without checking compatibility. Start normally from Steam.

## Windows source build

This GitHub project is a **source overlay**, not a redistribution of Unreal Engine SDK dumps or commercial game files.

1. For development, install Git, Rust 1.99.1 and the Visual C++ MSVC build tools.
2. Run `scripts\prepare-upstream.ps1` **from this source checkout** to clone the pinned upstream code and overlay our modified source.
3. In `upstream/`, run `cargo build --release --locked`.
4. After validation, the resulting `target/release/final_fantasy_resonance_fix.dll` may be renamed to `final_fantasy_resonance_fix.asi`.

GitHub Actions runs a Windows source build, dependency license collection, official ASI-loader provenance check and path/privacy-pattern checks. The Actions build artifact is a *developer verification artifact*, **not the fully packaged Nexus mod**. A clean compile or malware scanner result is not a universal safety guarantee.

## Testing and limitations

- At **3440 × 1440** on Windows Steam, the author tested the 1.50× DOF calibration in movement, interiors, town and world map, and reported several hours without recurrence of the earlier crash; the combined v1.1.0 release archive was explicitly approved for publication. This does not guarantee crash-free operation on every system.
- The controller **Y** button may occasionally fail to reopen a system menu on the first press. This also happens in the **unmodded** game, although some experimental modded runs seemed to have it more frequently; that difference has not been explained. Keyboard ESC did not exhibit it in the same tests. This mod does not hook gamepad input.
- Other aspect ratios, 32:9, different game builds and future updates have not been fully validated. Native hooks may break following game updates.
- On 16:9, upstream/native hooks may still be installed even if ultrawide offset is zero.

## Credits and licensing

- **PolarWizard / Dominik Protasewicz** — original fix, MIT.
- **Darjjj** — battle transition scratch-target correction ([upstream PR #3](https://github.com/PolarWizard/final-fantasy-resonance-fix/pull/3)).
- **ThirteenAG** — [Ultimate ASI Loader](https://github.com/ThirteenAG/Ultimate-ASI-Loader), MIT (distributed in the Nexus ZIP, not in the source overlay).
- **RimuModding community extension** — minimap/HUD positioning, menu transitions, optical DOF compensation, crash-guard integration, manual preset packaging and documentation.

Applicable notices are in `LICENSES/`, the Nexus ZIP and [THIRD_PARTY_DEPENDENCIES.md](THIRD_PARTY_DEPENDENCIES.md). This is an unofficial community project, unaffiliated with Square Enix, other rights holders or the original authors. Development included AI-assisted source and documentation.
