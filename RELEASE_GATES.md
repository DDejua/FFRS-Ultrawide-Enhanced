# Checks required before a public binary release

## Verified so far (2026-10-24)

- [x] Clean Steam game uninstall/reinstall, initial unmodded launch, and one-click install of the GitHub CI-build candidate (user-reported).
- [x] HYBRID minimap presentation tested at 3440×1440 using that CI-build candidate (user-reported).
- [x] ULTRAWIDE, CLASSIC, and HYBRID modes tested successfully with the new GitHub-build candidate; selector switches between all three (user-reported).
- [x] Several battles and battle transitions on the new candidate without unexpected crashes (user-reported). This is a meaningful stability test, not a zero-crash guarantee.
- [x] 16:9 streaming test on the new candidate succeeded (user-reported).
- [x] Pinned upstream 0.3.0 and Rust 1.99.1 Windows CI build succeeded: [run 40055394902](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/actions/runs/40055394902).
- [x] Targeted CI binary scan for local Windows user-profile paths passed (run 40055394902).

## Open release blockers

- [ ] Obtain and review full notices for **all 38 external Cargo crates**, not just license identifiers; include applicable notices in the final distributed package.
- [ ] Confirm precisely which Ultimate ASI Loader binary/version is being redistributed and audit the matching license notices / provenance.
- [ ] Confirm rights and attribution for any generated SDK or other assets, and publish source rebuild instructions that do not inadvertently redistribute restricted content.
- [ ] Run a comprehensive privacy scan of every file of the FINAL proposed Nexus ZIP (source, archives, scripts, compiled binary, metadata, any hidden folders), beyond the targeted CI profile-path check.
- [ ] Complete binary/security checks suitable for an executable mod distribution and validate downloaded release hashes.
- [ ] Publish attributable source in a public repository or equivalent accessible location, after final privacy and license review; current source repository is PRIVATE.
- [ ] Review the final Nexus description, install/uninstall instructions, gallery and downloadable archive, then publish only after release blockers are closed.

## Compatibility limits to communicate

- Validated by one tester on Windows Steam at 3440×1440, plus a 16:9 streaming configuration.
- Other aspect ratios, game builds and 32:9 remain community-test targets.
- No game mod can be guaranteed crash-free on every system.
