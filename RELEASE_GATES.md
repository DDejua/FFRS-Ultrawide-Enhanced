# FFRS Ultrawide Enhanced v1.0.1 — release readiness (2026-10-24)

## Proven upstream v1.0.0 baseline

- The original v1.0.0 package used an in-game-tested CI ASI: SHA256 `f9c7dc834924a13f7c46a3ac9652bfcc954d5041b98cd6c7c8467dcce49817b6`, from [run 40055394902](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/actions/runs/40055394902).
- Original Steam testing covered automatic install, three HUD modes, combat and 16:9 streaming at 3440×1440.
- Bundled `winmm.dll` was verified to match ThirteenAG's official x64 release SHA256 `900fa114af8317bfe22015ca77becc1afa9853831213ff17a5e03e0f45d58937`.
- Rust dependency notices and license provenance were collected from pinned CI sources.

## v1.0.1 changes verified so far

- [x] HYBRID minimap stays physically at the wide edge while suppressed by RenderOpacity in the system/mission menus.
- [x] Quest overlay left-side leak suppressed.
- [x] Menu-close grace restores both elements while idle, after about 160 ms, via the game-thread frame callback.
- [x] A lighter candidate with `probe=false` was compiled and tried in game. No improvement to occasional controller-Y missed menu reopening.
- [x] The controller-Y issue also occurs without the mod, though the tester reported a lower frequency unmodded. This difference has not been root-caused.
- [ ] Smoke-test the **exact packaged** final 1.0.1 ZIP with manual + scripted installation and HYBRID / CLASSIC / ULTRAWIDE before Nexus publishing.
- [ ] Complete Nexus Mods security/quarantine review of the mod page download and confirm whether support recommends replacing the quarantined file.

## Compliance / release procedure

- [ ] Verify the final GitHub Actions Windows build and matching hash-manifest, loader provenance and full third-party license notices.
- [ ] Keep the existing Nexus mod page [#71](https://www.nexusmods.com/finalfantasyresonance/mods/71). Do not delete quarantined v1.0.0 file while awaiting support guidance.
- [ ] Ask Nexus support in ticket #273831 whether the v1.0.1 file can be evaluated directly instead of the quarantined v1.0.0 file.
- [ ] Once approved to proceed, upload the v1.0.1 ZIP *to the existing page*, edit changelog, confirm file links and published SHA-256.

## Limitations

Native DLL/ASI injection is inherently privileged. Binary compilation, hashes and malware scanners do not guarantee safety; some antivirus checks may quarantine a legitimate binary. No unlicensed game binaries or SDK dumps are distributed. Native hooks may break following game updates. Additional aspect ratios and game versions require independent validation.
