# Dependency licensing review — first pass (2026-10-24)

## Source of data

GitHub Actions run [40055394902](https://github.com/DDejua/FFRS-Ultrawide-Enhanced/actions/runs/40055394902) produced `dependency-metadata.json` from `cargo metadata --format-version 1 --locked` on the rebuilt pinned upstream source. The 38 external registry packages in that artifact declare SPDX license expressions.

## Initial observations (not a distribution clearance)

- The dependency set predominantly declares **MIT OR Apache-2.0**.
- `fern`, `grapnel`, `iced-x86`, and `winnow` declare **MIT**.
- `bytemuck` declares **Zlib OR Apache-2.0 OR MIT**.
- `unicode-ident 1.0.24` declares **(MIT OR Apache-2.0) AND Unicode-3.0**. The Unicode-3.0 component is an additional requirement, not an alternative.
- The remaining packages in the generated metadata declare the MIT/Apache-2.0 alternatives.

These are package-declared metadata values only. They do **not** substitute for the applicable full license texts and required copyright notices.

## Before publication

1. Collect the actual license files from each pinned package version, keeping any required copyright statements (not only generic license templates).
2. Ensure the Unicode-3.0 text/notice is included for `unicode-ident`.
3. Cross-check complete notices for the original PolarWizard code and distributed Ultimate ASI Loader.
4. Keep generated third-party notices in the final end-user ZIP and review that distribution manually.
5. Do not distribute the Windows executable publicly until all license and provenance checks in `RELEASE_GATES.md` are complete.

This document is a development inventory and is **not legal advice or an approval to publish**.
