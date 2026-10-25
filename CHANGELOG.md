# Changelog

## 1.1.0 — 2026-10-25 (public Nexus release)

**New:**
- Automatic, default-on ultrawide depth-of-field (DOF) compensation without camera zoom.
- Resolution-aware bounded sensor-width scale from a two-anchor mathematical model: unchanged at 16:9; **1.50× calibrated and user-tested at 3440×1440**.
- Separate `[post_processing] ultrawide_dof_fix = false` to disable only the extra ultrawide compensation; `depth_of_field = false` disables all DOF.
- Theoretical auto-scaling for 32:9 and larger, **pending real-hardware testing**; no claim of physically exact DOF outside the user-tested resolution.

**Fixed:**
- Encounter capture state guard consumes a pending request when capture already matches the viewport, avoiding stale state during later exploration; tested for multiple hours without recurrence of the reported crash, but cannot guarantee full stability.

**Preserved and distributed:**
- HYBRID (default), ULTRAWIDE, CLASSIC HUD layouts, ultrawide gameplay and 16:9 presentation for pre-rendered FMVs.
- Manual-only release archive: no CMD, PowerShell, BAT or other executable installer scripts.
- Recommended display mode **Borderless Windowed**; fullscreen exclusive may cause stretching.

**Approved Windows ASI:** `7c9b091522ad8e4606895218f34690551254e0b5c13426abbfefd53a057015d1` (SHA-256).

**Approved Nexus ZIP:** `FFRS Ultrawide DOF Fix V1.1_RimuModding.zip`, SHA-256 `a3abfbe40a0e55002876fa988e67896ad1eccc8fc52adc4e69df546dcc2d26b0`. Source built at `0229fcb3d93fd6616a691c69666a7d8721b20ba5`, Windows CI [#40189222369](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/actions/runs/40189222369). Later documentation merges do **not** alter this tested binary.

[Download v1.1.0 on Nexus Mods](https://www.nexusmods.com/finalfantasyresonance/mods/71?tab=files) · [DOF model details](docs/DOF_AUTO_FORMULA.md)

## 1.0.1 — 2026-10-24 (public Nexus release)

- Fix HYBRID minimap overlay bleeding into system and mission menus.
- Fix field mission objective overlay bleeding across the left edge of menus.
- Hide HUD overlays using their RenderOpacity instead of shifting the minimap across the screen.
- Restore overlays from the game thread after a brief 160 ms grace window, without waiting for character movement.
- Cancel the pending HUD restore when another menu opens.
- Remove broad diagnostic logging for production-oriented testing, retain minimal status logs.
- Keep `[minimap] probe = false` as the safe default, without disabling quest overlay protection.
- Public distribution uses **manual installation only**; no `.cmd` or `.ps1` installer scripts. Includes three manual TOML HUD profiles and the revised `README.txt`.

See [release notes](NEXUS_RELEASE_NOTES_v1.0.1.md) for testing scope and the separate controller Y-button issue observed in the unmodded game.

## 1.0.0 — 2026-10-24

- Initial release: ULTRAWIDE, CLASSIC and HYBRID HUD presets.
- Resolution-aware HYBRID minimap positioning, 16:9 cinematic corrections, and battle transition support.
- Manual and script-based installation, source overlay and license notices.
