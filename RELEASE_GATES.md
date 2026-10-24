# FFRS Ultrawide Enhanced v1.0.0 — release readiness (2026-10-24)

## Complete: functional testing and release packaging

- [x] User-tested the precise CI-built ASI from run [40055394902](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/actions/runs/40055394902), SHA256 `f9c7dc834924a13f7c46a3ac9652bfcc954d5041b98cd6c7c8467dcce49817b6`.
- [x] Clean Steam game reinstall and unmodded game boot succeeded, followed by clean automatic installation.
- [x] HYBRID, ULTRAWIDE and CLASSIC modes, automatic selector, several encounters and 16:9 streaming tested successfully on Windows Steam, 3440×1440 (user report).
- [x] Full license texts copied from the pinned 38 Cargo crates in GitHub CI [run 40057315301](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/actions/runs/40057315301). `iced-x86 1.21.0` MIT notice is taken from the matching official project tag because the registry tarball lacks it. `unicode-ident` Unicode-3.0 notice is present.
- [x] Included `winmm.dll` matches the exact official ThirteenAG x64-latest asset; CI compared the official archive digest and DLL SHA256 `900fa114af8317bfe22015ca77becc1afa9853831213ff17a5e03e0f45d58937`.
- [x] Final end-user ZIP organized with two Windows launchers, an offline manual installation method, three TOML presets, only required runtime files and applicable legal notices.
- [x] Release ZIP integrity, SHA-256 manifest, unmodified tested ASI bytes and file-level privacy pattern checks passed in packaging audit. These checks do not establish complete malware or universal privacy guarantees.
- [x] User-facing Nexus page draft and six authentic, aspect-ratio-preserving annotated screenshots prepared, separately from the end-user ZIP.

## Remaining to publish

- [ ] Change the GitHub repository from **private** to **public** when ready. The available GitHub connection does not support changing repository visibility.
- [ ] Create/publish the Nexus Mods page, upload the manually installable ZIP and its screenshots. There is no connected Nexus publishing integration in this session.
- [ ] Let Nexus complete its server-side malware/security checks on the uploaded binary; review any quarantine or warning.
- [ ] Confirm the public page links, credit/licensing text, tags and supplied archive's published SHA256.

## Limits

- Tested on one hardware/software configuration. Other display resolutions, 32:9, storefronts and future game versions are not guaranteed.
- The distribution includes binary code running inside the game; no binary scanner can guarantee it is risk-free.
- Full game files and generated Unreal SDK files are not redistributed; source preparation clones the upstream public MIT project.
- Changes to build scripts/CI for licensing after the user-tested build did NOT replace its proven ASI in the final ZIP.
