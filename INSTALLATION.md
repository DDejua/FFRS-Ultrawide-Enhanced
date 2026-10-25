# FINAL FANTASY RESONANCE — ULTRAWIDE FIX + DOF

**Version 1.1.0 · Windows / Steam · Manual installation only**

[Download from Nexus Mods](https://www.nexusmods.com/finalfantasyresonance/mods/71?tab=files) · [Changelog](CHANGELOG.md)

**IMPORTANT: Use Borderless Windowed / Windowed Fullscreen.** Exclusive fullscreen can stretch the output on ultrawide displays. This is an unofficial community mod.

## New installation

1. Close the game completely and **extract the downloaded ZIP**.
2. In **Steam > Library**, right-click FINAL FANTASY RESONANCE > **Manage > Browse local files**.
3. Open `FFRS\Binaries\Win64`; confirm `FFRS-Win64-Shipping.exe` is in that folder.
4. Copy `winmm.dll` and the entire `scripts` folder from the ZIP into that **Win64** folder.
5. Start the game from Steam and select **Borderless Windowed** in the game's display settings.

Correct installation layout:

```text
FFRS/
  Binaries/
    Win64/
      FFRS-Win64-Shipping.exe
      winmm.dll
      scripts/
        final_fantasy_resonance_fix.asi
        final_fantasy_resonance_fix.toml
```

**Default:** HYBRID HUD and automatic ultrawide DOF are enabled without extra configuration. There are **no CMD, PowerShell or executable installers** in the downloadable mod archive. The `scripts/` directory is the game's ASI plugin folder, not a command script.

## Update from v1.0.1 or earlier

1. Exit the game fully.
2. Back up your existing `scripts/final_fantasy_resonance_fix.asi` and `scripts/final_fantasy_resonance_fix.toml` **outside the game folder**. Also back up your loader if you intend to change it.
3. Extract the v1.1.0 ZIP.
4. Copy the new `scripts` folder into `FFRS\Binaries\Win64`, replacing **only this mod's** existing ASI and TOML files.
5. **Keep your existing compatible `winmm.dll`**. Never blindly overwrite another mod's loader (`winmm.dll`, `dxgi.dll` or `dwmapi.dll`).
6. If you had custom TOML settings, reapply them to the **new** TOML so the new defaults, including DOF compensation, are present.
7. Start the game from Steam in **Borderless Windowed** mode.

You do **not** need to reinstall the game or run any command.

## HUD presets

Change only `final_fantasy_resonance_fix.toml`, not the ASI or loader:

1. Exit the game.
2. Open `HUD_PRESETS/` in the downloaded ZIP.
3. Choose **HYBRID** (default; 16:9-centered HUD with minimap at ultrawide edge), **ULTRAWIDE** (21:9 HUD) or **CLASSIC** (16:9-centered HUD and minimap).
4. Copy the selected preset TOML into `FFRS\Binaries\Win64\scripts`, replacing the previous TOML. Reapply any customizations.
5. Restart the game.

A separate `[hud] constrain = 2` option selects the experimental 32:9 HUD root; 32:9 operation has not been independently validated.

## DOF controls

Open `FFRS\Binaries\Win64\scripts\final_fantasy_resonance_fix.toml` with the game **closed**. By default:

```toml
[post_processing]
depth_of_field = true
ultrawide_dof_fix = true
```

- Both `true`: game DOF **plus** automatic compensation for ultrawide resolutions.
- `ultrawide_dof_fix = false`: keep the game's **original DOF** without the ultrawide compensation.
- `depth_of_field = false`: **disable DOF entirely**, including compensation.
- At **16:9**, automatic compensation is 1.0× (no extra blur). At **3440×1440**, the calibrated multiplier is 1.50×.
- For **32:9 and wider** the mathematical formula extrapolates an **untested** factor; results need community feedback.

See [formula and limits](docs/DOF_AUTO_FORMULA.md). Fully restart the game after changing any TOML setting.

## Troubleshooting, rollback and removal

- The log is in `FFRS\Binaries\Win64\scripts\final_fantasy_resonance_fix.log`. At 3440×1440, check for `DOF_AUTO aspect=2.388889 scale=1.500000` when applicable.
- If the picture is stretched, check that the game's mode is **Borderless Windowed** instead of exclusive fullscreen.
- Before reporting a crash, note scene, resolution, HUD preset and steps to reproduce; retain the log.
- To revert to the previous version, close the game and restore your backups.
- To uninstall, remove **only** the files installed by this mod. Do not delete another mod's loader or unrelated game files.

Manual ZIP SHA-256 (published v1.1.0 archive): `a3abfbe40a0e55002876fa988e67896ad1eccc8fc52adc4e69df546dcc2d26b0`.

Source-derived ASI SHA-256: `7c9b091522ad8e4606895218f34690551254e0b5c13426abbfefd53a057015d1`.
