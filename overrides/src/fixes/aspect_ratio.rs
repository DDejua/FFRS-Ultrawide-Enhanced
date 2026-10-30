//! EXPERIMENTAL 16:10 FOV axis test only. Never package as stable release.
//! Diff against pinned PolarWizard source is restricted to the axis value + log.
//! Opens the rendered view up past the packaged aspect ratio.

use crate::utils::{ModuleInfo, patch, pattern_scan};

/// Stops the engine constraining the rendered view to the packaged aspect
/// ratio, and replaces the axis constraint the engine would otherwise apply.
///
/// Both are byte patches on code the engine has not run yet at DLL attach, so
/// neither needs the engine to be up and neither depends on a measurement.
pub(super) fn apply(module: &ModuleInfo) {
    // The match ends at AND ECX,1 reading the packaged ConstrainAspectRatio
    // flag; its immediate at +15 becomes 0, so the flag never survives the
    // mask and the constrained path is not taken.
    const ASPECT: &str = "89 ?? ?? 0F ?? ?? ?? ?? ?? 00 33 ?? ?? 83 ?? 01";
    if let Some(address) = pattern_scan(module.address, ASPECT) {
        patch(address + 15, "00");
        log::info!(
            "ConstrainAspectRatio: first match +{:X}, byte +15 = 00",
            address - module.address.0 as usize
        );
    } else {
        log::error!("ConstrainAspectRatio: signature not found");
    }

    // TEST ONLY / Issue #31: experimentally replace AspectRatioAxisConstraint
    // with MOV EDX,1 (MaintainXFOV) + 3 NOPs to preserve horizontal coverage
    // while the viewport grows vertically on 16:10 (1280x800).
    // DO NOT RELEASE OR USE ON 21:9 / 32:9; this is a global axis override
    // that will change the stable ultrawide camera behavior.
    // The 16:9 gameplay, movies, menus and combat paths require game testing.
    const AXIS: &str =
        "0F ?? ?? ?? ?? ?? 41 ?? ?? ?? ?? ?? 00 00 48 ?? ?? ?? ?? 00 00 4C ?? ?? 4D ?? ??";
    if let Some(address) = pattern_scan(module.address, AXIS) {
        patch(address + 6, "BA 01 00 00 00 90 90 90");
        log::info!(
            "AspectRatioAxisConstraint: first match +{:X}, TEST-ONLY MaintainXFOV patch applied (Issue 31, 16:10 trial)",
            address - module.address.0 as usize
        );
    } else {
        log::error!("AspectRatioAxisConstraint: signature not found");
    }
}
