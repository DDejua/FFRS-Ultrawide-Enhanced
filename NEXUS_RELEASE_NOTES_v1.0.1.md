# FINAL FANTASY RESONANCE — Ultrawide Enhanced v1.0.1

**Status:** Release candidate prepared on GitHub. Not yet approved or published on Nexus Mods.

## Changes from v1.0.0

- **HYBRID:** The ultrawide-positioned minimap no longer remains visible over the centered system/mission menus.
- The field quest/objective box is also suppressed while those menus are open, preventing its left edge from leaking into the menu.
- The minimap stays at its ultrawide coordinates instead of jumping between 16:9 and 21:9 positions during menu transitions.
- Both HUD elements return to their saved original opacity approximately 160 ms after the system menu closes. A game-thread per-frame check allows restoration when standing completely still; quickly reopening a menu cancels pending restoration.
- Removes noisy per-widget UI diagnostics while preserving the fix when `[minimap] probe = false`, as in the distributed HYBRID profile.
- The ULTRAWIDE and CLASSIC presets remain available.

## Installation

Download the *full end-user ZIP* and see its `README.txt` or [INSTALLATION.md](INSTALLATION.md). **Manual installation requires copying three files:** `winmm.dll` into `FFRS/Binaries/Win64/`, then `final_fantasy_resonance_fix.asi` and `final_fantasy_resonance_fix.toml` into `FFRS/Binaries/Win64/scripts/`. Back up any existing files and check ASI-loader compatibility first.

If upgrading from v1.0.0, close the game before replacing files. Automatic installation and switching HUD presets are optional convenience scripts. The default is HYBRID; game restart is required after changing presets.

## Testing and known limitations

- Core v1.0.1 menu HUD behavior was user-tested in game at 3440×1440 on Windows Steam, including quick transitions and restoration without moving.
- The **final rebuilt/repackaged v1.0.1 ZIP** should still undergo a smoke test of all three HUD modes and both installation methods before public Nexus publishing.
- The controller **Y** button can occasionally require another press to reopen the system menu, and this was reproduced **without the mod**. It was reported more frequently with the experimental mod in some tests; the cause of that difference remains unproven. ESC did not show the issue in the user's test.
- Other resolutions, game versions, hardware and 32:9 setups are not fully certified. Future game updates may invalidate native hooks.
- This is an unofficial third-party native DLL/ASI mod. The Windows loader is from ThirteenAG; original fix from PolarWizard. Inspect the source and license notices before installing.

## Sources

- [Community overlay](https://github.com/DDejua/FFRS-Ultrawide-Enhanced)
- [PolarWizard original fix](https://github.com/PolarWizard/final-fantasy-resonance-fix)
- [Darjjj's encounter fix proposal](https://github.com/PolarWizard/final-fantasy-resonance-fix/pull/3)
- [ThirteenAG Ultimate ASI Loader](https://github.com/ThirteenAG/Ultimate-ASI-Loader)
