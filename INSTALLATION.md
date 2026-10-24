# Manual installation (no scripts needed)

This guide explains how to install **FFRS Ultrawide Hybrid Fix v1.0.1**, as distributed on [Nexus Mods](https://www.nexusmods.com/finalfantasyresonance/mods/71?tab=files). **The public ZIP has no `.cmd` or `.ps1` installers. Installation is manual only:** copy the three runtime files described below.

## 1. Extract the download

Close FINAL FANTASY RESONANCE. Right-click the downloaded ZIP and choose **Extract All**. Open the extracted folder.

## 2. Find the correct game folder

In Steam, open **Library → right-click FINAL FANTASY RESONANCE → Manage → Browse local files**.

In File Explorer, open **FFRS → Binaries → Win64**. Verify that you can see **`FFRS-Win64-Shipping.exe`**. This is the destination folder; its full drive path depends on where Steam installed the game.

## 3. Back up any existing files

Before replacing files, back up any existing `winmm.dll`, `scripts/final_fantasy_resonance_fix.asi`, and `scripts/final_fantasy_resonance_fix.toml`.

**If you use other mods with an ASI loader** (`winmm.dll`, `dxgi.dll`, or `dwmapi.dll`), check compatibility. Do not blindly overwrite or combine loaders.

## 4. Copy exactly three files

From the **extracted mod** folder:

1. Copy **`winmm.dll`** into the game's **Win64** folder, next to `FFRS-Win64-Shipping.exe`.
2. Open or create a folder called **`scripts`** inside **Win64**. Copy **`final_fantasy_resonance_fix.asi`** and **`final_fantasy_resonance_fix.toml`** from the mod's `scripts` folder into that game folder.

The result should look like this:

```text
FFRS/Binaries/Win64/
├── FFRS-Win64-Shipping.exe       (game file — already there)
├── winmm.dll                     (mod)
└── scripts/
    ├── final_fantasy_resonance_fix.asi
    └── final_fantasy_resonance_fix.toml
```

You **do not** have to copy `HUD_PRESETS`, `LICENSES`, `_support` or `README.txt` into the game folder. They remain in your extracted download for mode selection, credits and verification. No automatic installer scripts are included.

## 5. Launch the game

Start normally through Steam. The default HUD mode is **HYBRID**: a centered 16:9 interface with the minimap placed at the ultrawide edge. In v1.0.1, the minimap and quest HUD are temporarily hidden while the system menu is displayed, and restored after closing it.

## Changing HUD mode manually

1. Exit the game.
2. Open **`HUD_PRESETS`** in your extracted mod folder.
3. Open the **`HYBRID`**, **`ULTRAWIDE`**, or **`CLASSIC`** folder:
   - **HYBRID (default):** centered 16:9 HUD, minimap at ultrawide edge.
   - **ULTRAWIDE:** 21:9 HUD with the minimap at the ultrawide edge.
   - **CLASSIC:** centered 16:9 HUD with the centered minimap.
4. Copy that folder's **`final_fantasy_resonance_fix.toml`** into the **game's** `FFRS/Binaries/Win64/scripts` folder, replacing the previous TOML (back it up if customized).
5. Launch the game again.

Only the TOML settings file changes between HUD presets. **Do not replace the DLL or ASI just to switch modes.**

## Uninstall manually

Close the game first. Delete **only** this mod's files if they were added by this mod: `Win64/winmm.dll`, `Win64/scripts/final_fantasy_resonance_fix.asi`, and optionally `Win64/scripts/final_fantasy_resonance_fix.toml`. Restore backups if these paths existed before installing. Do not delete another mod's ASI loader or unrelated game files.

## Security and compatibility

No installation commands or scripts are required. The mod is an in-process third-party DLL/ASI plugin: copying files manually removes script execution but does not eliminate the inherent risks of running third-party native code. As of the October 24, 2026 upload, Nexus Mods displays the v1.0.1 file as **Safe to use** and enables manual download. This does not guarantee functionality or safety on all systems.
