# FINAL FANTASY RESONANCE — ULTRAWIDE HYBRID FIX

Version 1.0.1 | Windows (Steam) | Manual installation only

Unofficial community mod. No installer or command scripts required.

## INSTALLATION

### FIND THE GAME'S CORRECT WIN64 DIRECTORY

- Open Steam > Library.
- Right-click FINAL FANTASY RESONANCE > Manage > Browse local files.
- In the game folder, open FFRS > Binaries > Win64.
- Make sure you see FFRS-Win64-Shipping.exe here.
- Leave this folder open. Your Steam library may be on any drive.

Copy winmm.dll and scripts folder into:

`Win64`

### Launch the game.

- Launch FINAL FANTASY RESONANCE normally from Steam.
- HYBRID mode is selected by default: centered 16:9 HUD with the minimap
  positioned toward the right edge of your ultrawide display.

## HOW TO CHANGE HUD MODE

You only need to replace ONE settings file. Do not reinstall the mod.

1. Close the game completely.
2. In the downloaded ZIP, open HUD_PRESETS folder.
3. Choose one of these folders:
   - HYBRID (default): centered 16:9 HUD, minimap at ultrawide edge.
   - ULTRAWIDE: 21:9 HUD
   - CLASSIC: centered 16:9 HUD
4. Copy the final_fantasy_resonance_fix.toml from your chosen folder.
5. Paste it into the GAME'S FFRS\\Binaries\\Win64\\scripts folder and replace
   the existing file.

   (Back up your TOML first if you customized its settings.)

6. Start the game. Your chosen mode now applies.

Only the .toml settings file changes between modes. Leave winmm.dll and the
.asi plugin untouched. In the ULTRAWIDE preset, 21:9 HUD is used even on 32:9;
an additional 32:9 HUD option can be selected manually in TOML (constrain=2).

## UNINSTALLING

Close the game. Remove only the files this mod installed:

```text
Win64\\winmm.dll
Win64\\scripts\\final_fantasy_resonance_fix.asi
Win64\\scripts\\final_fantasy_resonance_fix.toml (if you want)
```

Restore your backups if those paths existed before installing this mod.
Do not delete another mod's ASI loader or unrelated game files.
