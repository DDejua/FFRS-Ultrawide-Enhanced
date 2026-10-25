# DOF Investigation — Issue #5 (isolated, unreleased diagnostic)

**State:** SOURCE DIAGNOSTIC ONLY. Not a gameplay-confirmed fix, not part of the stable v1.0.1 Nexus upload. **Do not replace the user's ongoing crash-stability candidate.**

## Observed reproduction (2026-10-25)
- Windows Steam; 3440×1440 monitor; D3D12; graphics/postprocess quality 3.
- Same world-map location beside castle and waterfall. In original 16:9 presentation with `super_enable=false`, distant landscape shows strong depth blur.
- In ultrawide 3440×1440 with `super_enable=true`, distant mountains, trees and paths are visibly sharper / blur is greatly reduced.
- With `super_enable=true` and **2560×1440 (16:9) windowed**, blur reappears (confirmed by screenshot and user observation). Fullscreen 16:9 appears stretched on a 21:9 monitor, likely display scaling, and is **not** the basis for the DOF comparison.
- Shipped `[post_processing] depth_of_field = true` and `sg.PostProcessQuality=3`; no user-created Engine.ini found. **Unresolved:** whether the view-projection adjustment changes effective DOF or a renderer path otherwise differs between aspect ratios.

## Diagnostic source design
- Branch: `test/dof-diagnostics-issue5` from `develop`; linked Issue #5. Do not merge this branch or build it into a public release.
- New `[post_processing] dof_probe = false` flag is **off by default**. With every effect enabled and probe off, the postprocess hook is not installed (unchanged shipped path).
- With `dof_probe = true`, install the existing pinned game's final-postprocessing hook and record **read-only** `FPostProcessSettings` fields on first view and every 600 views: `DepthOfFieldFocalDistance`, `DepthOfFieldFstop`, `DepthOfFieldScale`, `DepthOfFieldDepthBlurAmount`, `DepthOfFieldDepthBlurRadius`.
- The diagnostic **does not repair DOF**, change aperture/focal distance/FOV, or force a rendering effect. Enabling the optional native hook adds crash risk and must only be tested after the separate castle-crash stability candidate is validated.
- A logged view might not be the main gameplay camera (e.g., scene captures or secondary views). This telemetry alone **cannot establish** whether the problem comes from the projection matrix, focal parameters, or a shader path.
- Do not add/copy UE4SS or third-party mod files; this research uses only our existing upstream/native source.

## Future test procedure (NOT YET APPROVED FOR GAMEPLAY)
1. First finish and record stability testing of Issue #4's **separate** ASI (`test/encounter-pending-reset-v102`). Keep its exact binary/hash and do not overwrite it.
2. After GitHub Windows CI passes on this DOF branch, record its source commit SHA, CI run and built ASI SHA-256. This diagnostic binary is *different* from the crash candidate.
3. In a backed-up isolated test installation, start with **DOF diagnostic build only**, never both candidate ASIs. Preserve the other build for rollback. Add `dof_probe = true` under `[post_processing]` in its TOML while keeping `depth_of_field = true`. Restart the game.
4. Record the same world-map scene in 2560×1440 windowed and 3440×1440, without changing graphics settings. Note mode/resolution per run, save `scripts/final_fantasy_resonance_fix.log`, and take before/after screenshots. Turn the probe off when finished.
5. If a crash occurs, stop this experiment. Keep any private crash dump locally; post only sanitized exception, game/module offset and log lines.
6. Compare scene-relevant samples. If numeric settings match but blur changes, investigate view/projection and engine shader behavior rather than arbitrarily amplifying DOF.

## Merge gate
- Preserve the Issue #4 memory-safety candidate as an **independent patch** until user gameplay validation.
- DOF code must be tested on its own, reviewed via PR into `develop`, then combined with **all validated** fixes in a new SHA-identified build before any release.
- `main` and Nexus v1.0.1 stay unchanged.
