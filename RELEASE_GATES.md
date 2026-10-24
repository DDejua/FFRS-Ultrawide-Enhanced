# Checks required before a public binary release

- [x] Functional Windows test at 3440×1440 of the 3 HUD modes (user report).
- [x] Multi-hour combat-transition test of **previous binary candidate** (user report).
- [x] Auto-install, mode selector and backup creation for **previous clean RC** (user report).
- [ ] Build fresh clean `.asi` in Windows CI using pinned upstream and Rust.
- [ ] Review the CI-produced `.asi` for path leaks and other metadata.
- [ ] Test that **specific freshly built `.asi`**, including modes, encounter transitions,
      16:9, movie framing and HUD preset changes. The old tests do not transfer automatically.
- [ ] Identify licensing notices required for the 38 external Cargo crates; include them
      in the binary's end-user distribution (not only an inventory of license IDs).
- [ ] Confirm the Ultimate ASI Loader binary version corresponds to the named source/
      license. Ideally download directly from its official release in the packaging phase.
- [ ] Confirm any additional game SDK/assets redistribution obligations (this source
      overlay doesn't include the generated Unreal SDK).
- [ ] Run static privacy scan on ALL release files, including source, archives, scripts,
      and compiled DLL; check paths, emails, metadata, names, and executable resources.
- [ ] Final malware/security assessment of distributed Windows binaries.
- [ ] Publish a source repo with license and contributor attribution.
- [ ] Confirm release download hash and create Nexus page.

No green checkbox above is a substitute for testing on other user machines.
