# FFRS Ultrawide Hybrid Fix v1.0.1 — public-release verification (2026-10-24)

## Public status

- [x] **Published:** [Nexus Mods page #71 — Files](https://www.nexusmods.com/finalfantasyresonance/mods/71?tab=files).
- [x] The uploaded **v1.0.1 main file** is shown by Nexus as **Safe to use** and offers **Manual download**, based on the maintainer's October 24, 2026 screenshot.
- [x] The publicly submitted end-user archive is **manual-install only**, with no `.cmd` or `.ps1` scripts.
- [x] The ZIP contains `winmm.dll`, `scripts/final_fantasy_resonance_fix.asi`, `scripts/final_fantasy_resonance_fix.toml`, three preset TOMLs in `HUD_PRESETS`, `README.txt`, license notices and support build/hashes.
- [x] The published-package preparation audit checked ZIP integrity, verified that `README.txt` exactly matched the maintainer-supplied README, and confirmed that `CHANGELOG.txt`, `.cmd`, and `.ps1` files were absent.
- [x] The locally prepared archive, named `FFRS Ultrawide Hybrid Fix v1.0.1.zip`, had SHA-256 `82924d2fdf0cbb86cdffdb3bf9ce8550ca9df512d3ff1db718cf80627450a66f`. **This is the local package digest; we have not independently downloaded the server-hosted Nexus copy to compare hashes.**
- [x] GitHub source changes merged to `main` via [PR #1](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/pull/1). The [final candidate Windows CI run](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/actions/runs/40078489074) passed compilation, loader provenance, license and static path checks.

## Functional testing and limitations

- [x] During development the maintainer tested game startup, HYBRID/ULTRAWIDE/CLASSIC, combat and 16:9 streaming under Windows Steam at 3440 × 1440.
- [x] v1.0.1 HYBRID minimap and field quest box are temporarily hidden during system menus without jumping; they return at roughly 160 ms even while idle, as confirmed in experimental playtesting.
- [x] Occasional missed system-menu Y-button reopening also reproduced in the **unmodded game**. The maintainer reported the problem far less frequently without the mod. Mod-specific aggravation remains possible but has not been demonstrated or traced. ESC behaved normally in testing.
- [ ] **Separate post-packaging in-game smoke test of the exact publicly submitted manual-only v1.0.1 ZIP** has not yet been confirmed.
- [ ] Independent compatibility tests for other resolutions, 32:9, hardware and future game patches remain open.

## Distribution and support

- Distribution is **manual-only**. The instructions are in the ZIP's `README.txt` and this repository's [INSTALLATION.md](INSTALLATION.md). Users need only copy the loader, ASI and default TOML into the stated Win64 paths.
- The source repository's PowerShell build/audit scripts are **developer tools** and not bundled with the Nexus user archive.
- Nexus support ticket **#273831** was created before the first quarantine. The later manual-only v1.0.1 package passed the Nexus scanner. Ticket closure is not separately confirmed.
- Do not claim that a successful malware scan guarantees safety, or that tested coverage extends to every aspect ratio and game version.

## Provenance

- Original upstream [PolarWizard Fix 0.3.0](https://github.com/PolarWizard/final-fantasy-resonance-fix) pinned to commit `0f5bc2c4bab2b1680c59d8d1c2135253ae14c0df`.
- Official ThirteenAG x64 `winmm.dll` was fingerprint-verified during build/packaging, SHA-256 `900fa114af8317bfe22015ca77becc1afa9853831213ff17a5e03e0f45d58937`.
- MIT license notices and pinned Rust dependency notices are carried in the published archive.
