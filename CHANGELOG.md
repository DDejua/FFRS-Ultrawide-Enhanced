# Changelog

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
