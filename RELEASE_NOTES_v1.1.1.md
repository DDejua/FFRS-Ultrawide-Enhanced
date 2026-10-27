# FINAL FANTASY RESONANCE — Ultrawide Fix + DOF v1.1.1

**Incremental Vision FMV transparency correction** · 27 October 2026 · RimuModding

## Fixed
- Restores the live battle environment behind *transparent* intros for certain Vision animations, including **Cloud** and **Terra**, instead of drawing an opaque black background across the movie.
- Retains solid black **outer** bars outside the centered 16:9 film. The bands are derived from the current viewport rather than hard-coded pixel positions.
- Standalone FMVs such as **Bahamut** and **Warrior of Light** remain visually unchanged.

## Preserved from v1.1.0
- Automatic ultrawide **depth-of-field compensation** (and both DOF opt-out settings).
- The encounter capture pending-state stability correction.
- The HYBRID, ULTRAWIDE and CLASSIC HUD presets; minimap/menu/quest UI restoration.
- Correct display of *story* FMVs and all other existing ultrawide fixes.
- The identical v1.1.0 vetted **ThirteenAG winmm.dll** (SHA-256 `900fa114af8317bfe22015ca77becc1afa9853831213ff17a5e03e0f45d58937`).
- Completely manual installation. **No executable installation scripts.**

## Testing and limits
- **3440×1440:** Cloud visually tested in all three HUD presets; Terra, Bahamut and Warrior of Light tested; story FMVs tested by the author. No visible regression found in these sequences.
- **32:9 and wider:** the side-band geometry scales automatically in code, but **no physical 32:9 setup was available**. Community feedback is welcomed. This is *not* a declaration that all ultrawide ratios or every Vision are hardware-validated.
- Stability: the original v1.1.0 crash mitigation is preserved; rare crashes cannot be ruled out.

## Updating from v1.1.0
1. Close the game.
2. Back up your existing `FFRS/Binaries/Win64/scripts/final_fantasy_resonance_fix.asi`.
3. Replace **only that .asi** with the new one. Leave `winmm.dll` and your existing `.toml` unchanged.
4. Ensure `[movies] enable = true` and start the game.

**New installation:** follow the manual instructions inside the ZIP. Keep other mods' ASI loaders intact if present.

Please include your resolution, HUD preset and a sanitized log when reporting a problem.

**GitHub source:** [DDejua/FFRS-Ultrawide-Enhanced](https://github.com/DDejua/FFRS-Ultrawide-Enhanced) · **Nexus Mods:** [mod #71](https://www.nexusmods.com/finalfantasyresonance/mods/71)

Credits: PolarWizard (original fix), Darjjj (encounter transition contribution), ThirteenAG (ASI Loader), RimuModding (extensions). Licenses/attributions in ZIP.
