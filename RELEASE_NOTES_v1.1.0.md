# Final Fantasy Resonance — Ultrawide Fix + DOF v1.1.0

**Published to Nexus Mods:** 25 October 2026  
**Author:** RimuModding · **Target:** Windows / Steam  
**Download:** https://www.nexusmods.com/finalfantasyresonance/mods/71?tab=files

## Highlights

- **Automatic ultrawide depth-of-field** compensation enabled by default. The model preserves native 16:9 optics (1.0×) and matches the author's visual testing at **3440×1440 (1.50×)** without camera zoom.
- **Adaptive scaling** for other aspect ratios. 32:9 and wider predictions are a **mathematical extrapolation, not yet validated on 32:9 hardware**.
- **Encounter-transition stability:** consumes a pending capture when its aspect already matches the viewport; several hours of author gameplay testing without recurrence of the reported crash.
- **Player options:** `[post_processing] ultrawide_dof_fix = false` restores unmodified game DOF; `depth_of_field = false` removes DOF entirely.
- **HUD modes retained:** HYBRID (default), ULTRAWIDE and CLASSIC. Original 16:9 handling of pre-rendered FMVs remains.
- **Manual installation only:** copy `winmm.dll` and `scripts/` to `FFRS\Binaries\Win64`. **No CMD/BAT/PowerShell installer in the player ZIP.** **Borderless Windowed** is recommended rather than exclusive Fullscreen to prevent stretching.

## Upgrading from v1.0.1

With the game fully closed, back up your existing ASI/TOML and copy only the new `scripts/` files into `FFRS\Binaries\Win64`, replacing the old ASI/TOML. **Keep an existing compatible `winmm.dll`**. Reapply any custom TOML preferences; the new defaults enable DOF compensation automatically. No uninstall or reinstall is needed.

See [INSTALLATION.md](INSTALLATION.md) and the [precise two-anchor mathematical model](docs/DOF_AUTO_FORMULA.md).

## Release provenance

The public Nexus ZIP is **`FFRS Ultrawide DOF Fix V1.1_RimuModding.zip`**.

| Artifact | SHA-256 |
|---|---|
| Public ZIP | `a3abfbe40a0e55002876fa988e67896ad1eccc8fc52adc4e69df546dcc2d26b0` |
| Game ASI | `7c9b091522ad8e4606895218f34690551254e0b5c13426abbfefd53a057015d1` |
| Official ASI loader `winmm.dll` | `900fa114af8317bfe22015ca77becc1afa9853831213ff17a5e03e0f45d58937` |

- **Compiled source commit:** `0229fcb3d93fd6616a691c69666a7d8721b20ba5` ([Windows CI 40189222369](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/actions/runs/40189222369)).
- **Integration work:** [PR #18](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/pull/18).
- The public binary was approved by the author and remains **unchanged** from the candidate; follow-up documentation/release commits do not change the approved ASI.
- Presets, README, manual install instructions, third-party MIT licenses and Rust notices are included in the Nexus archive.
- A GitHub Actions **developer** build artifact is not the final manual-install Nexus ZIP.

## Remaining limitations

- The 21:9 calibration was verified in selected gameplay scenes. Not every cinematic camera or focus distance was individually verified.
- 32:9 (e.g. 5120×1440) uses a bounded theoretical scaling factor of approximately **2.58883×**, and **needs community testers**.
- Game updates or interactions with other graphics patches / ASI loaders can break native hooks. No native mod can guarantee zero crashes.

## Credits

- PolarWizard — upstream FFRS fix v0.3.0, MIT
- Darjjj — original encounter/battle transition contribution (PR #3)
- ThirteenAG — Ultimate ASI Loader, MIT
- RimuModding — ultrawide HUD improvements, DOF calibration/integration and encounter stability

This is an unofficial community project, unaffiliated with Square Enix. See [licensing](THIRD_PARTY_DEPENDENCIES.md) for details.
