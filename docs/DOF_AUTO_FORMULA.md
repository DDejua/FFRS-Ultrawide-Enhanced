# Resolution-aware depth of field — v1.0.2 candidate (Issues #4 / #5)

**Status:** experimental integrated candidate. NOT published, NOT tested on 32:9.

## User settings

```toml
[post_processing]
depth_of_field = true
ultrawide_dof_fix = true
```

- Both are `true` by default even with no TOML / older TOML.
- `depth_of_field = false` **disables all game DOF**, including any compensation.
- `ultrawide_dof_fix = false` retains the game's unmodified DOF for those who prefer it without disabling other fixes.
- At 16:9 or narrower the correction is **exactly 1.0**, preserving native game optics.
- Movie frame content (streamed/pre-rendered) is not rewritten by this option.

## Calibration and precise mathematical **model** (not precise optical science)

Let `A = viewport_width / viewport_height`, with `A0 = 16/9`.

The user validated the appearance of **1.50x optical sensor width at 3440x1440** after testing A/B screenshots, map, town, interiors and movement. The model is constrained to preserve two points exactly:

```text
S(A0) = 1.0
S(3440/1440) = 1.5

p = ln(1.5) / ln((3440/1440)/(16/9))
  = ln(1.5) / ln(43/32)
  = 1.3722985404457535...

S(A) = 1.0                               if A <= A0
S(A) = min(4.0, (A/A0)^p)                if A > A0
```

This is a mathematically exact *power-law interpolation through the two known points*; it does **not** follow necessarily from the renderer's physical optics. Values outside 21:9 are predictions. The cap prevents unbounded sensor modification in extreme formats.

| Native/render aspect | Example pixels | Predicted S | Evidence |
|---|---|---:|---|
| 16:9 | 1920x1080, 2560x1440 | 1.000 | Native optics unchanged |
| 43:18 | 3440x1440 | 1.500 | User-tested closest visual match |
| 12:5 | 3840x1600 | ~1.508 | Prediction, not tested |
| 32:9 | 5120x1440, 7680x2160 | ~2.58883 | Prediction, not tested |
| 48:9 | 7680x1440 | 4.0 bounded | Extrapolation clamped, untested |

The `S(A)` value scales *only* the already-blended `FPostProcessSettings.DepthOfFieldSensorWidth` for eligible gameplay views with finite sensor width and `0 < DepthOfFieldFocalDistance <= 10000`. Out-of-range focus values retain the game's original DOF. The user-tested V5 candidate had this same focus guard; it may not cover every cinematic camera. No FOV, aspect constraint, filmback projection, focus distance or f-stop patch is added.

## Aspect measurement

The original HUD root's `UWidgetLayoutLibrary::GetViewportSize` divided by viewport DPI scale supplies **the current in-game viewport aspect**, recorded by the existing HUD attachment/update callback. This is not the HUD *preset* ratio. The background postprocess hook reads the saved atomic width+height; it does not create a new view hook. If the main UI root is missing or the HUD feature is disabled, startup falls back to `ResolutionSizeX/ResolutionSizeY` in the **existing**, read-only `%LOCALAPPDATA%/FFRS/Saved/Config/Windows/GameUserSettings.ini`. If neither is valid, the DOF fix leaves the sensor unmodified.

**Limitations:** An external GPU scaling/stretch, dynamically changing resolution while HUD is disabled, or game streaming/render internals can deviate from saved resolution. HUD viewport aspect was demonstrated in this game for 2560x1440 (16:9) and 3440x1440 (43:18). Community 32:9 visual/runtime evidence is still required.

## Crash guard and rollback

The `PENDING.set(false)` no-crop consume branch from prior user-tested Issue #4 ASI is copied verbatim. CI compilation alone does **not** validate the rebuilt binary's runtime stability. Preserve a backup of prior ASI/TOML and do a multi-hour combined crash, menu, battle and cinema regression. If an issue appears, revert the candidate archive; do not blame the DOF sensor model without controlled A/B tests.

## Manual install distribution only

Candidate ZIP must contain no `.cmd`, `.bat`, `.ps1` files and no scripts to execute. `scripts/` is the game's **normal ASI plugin directory**, not a folder of runnable installers. Users only copy ASI and TOML. Developer-only source repository build scripts never ship in the player ZIP.
