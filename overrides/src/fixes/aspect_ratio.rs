//! Opens the rendered view up past the packaged aspect ratio.

use crate::utils::{ModuleInfo, patch, pattern_scan};

/// Stops the engine constraining the rendered view to the packaged aspect
/// ratio, and replaces the axis constraint the engine would otherwise apply.
///
/// Both are byte patches on code the engine has not run yet at DLL attach, so
/// neither needs the engine to be up and neither depends on a measurement.
pub(super) fn apply(module: &ModuleInfo, dof_test_original_axis: bool, dof_test_force_x_axis: bool) {
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

    // At +6, the eight bytes of MOVZX EDX,byte ptr [R15+0xB8] -- the load of
    // AspectRatioAxisConstraint -- become MOV EDX,0 plus three NOPs, which is
    // the maintain-Y constraint: a screen wider than 16:9 shows more to the
    // sides instead of cropping the top and bottom.
    const AXIS: &str =
        "0F ?? ?? ?? ?? ?? 41 ?? ?? ?? ?? ?? 00 00 48 ?? ?? ?? ?? 00 00 4C ?? ?? 4D ?? ??";
    // Explicit X-axis mode takes precedence over the old V3 "original" test
    // flag, and applies the same eight-byte instruction-shaped patch as Y,
    // with ONLY MOV EDX immediate changed from 0 to 1 (UE enum XFOV = 1).
    // This is an A/B diagnostic. Expect Vert- crops at 3440x1440.
    if dof_test_force_x_axis {
        log::warn!("DOF_AXIS_TEST mode=force_X axis_enum=1");
    } else if dof_test_original_axis {
        log::warn!("DOF_AXIS_TEST mode=original axis patch SKIPPED");
        return;
    } else {
        log::info!("DOF_AXIS_TEST mode=force_Y axis_enum=0");
    }

    if let Some(address) = pattern_scan(module.address, AXIS) {
        if dof_test_force_x_axis {
            patch(address + 6, "BA 01 00 00 00 90 90 90");
            log::warn!(
                "AspectRatioAxisConstraint: first match +{:X}, X-axis patch applied (TEST ONLY)",
                address - module.address.0 as usize
            );
        } else {
            patch(address + 6, "BA 00 00 00 00 90 90 90");
            log::info!(
                "AspectRatioAxisConstraint: first match +{:X}, Y-axis patch applied",
                address - module.address.0 as usize
            );
        }
    } else {
        log::error!("Constrain axis: signature not found; fail closed");
    }
}
