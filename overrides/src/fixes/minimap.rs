//! v3.1 - small upper-right corner nudge for the ultrawide minimap.
//! Only the outer WBP_MiniMapLayout_C is affected; descendants follow it.
//! Never change a new widget at 16:9. A previously shifted instance is
//! restored to its original translation when a later callback sees 16:9.

use std::{cell::RefCell, collections::{HashMap, HashSet}};

use sdk::{
    CoreUObject::FVector2D,
    UMG::{UUserWidget, UWidget, UWidgetLayoutLibrary},
};

use super::{
    layout::{OffsetState, apply_offset},
    widgets::{live, outward},
};

const MINIMAP_LAYOUT_CLASS: &str = "WBP_MiniMapLayout_C";
const BASE_ASPECT: f64 = 16.0 / 9.0;
// Additional space taken from the author's top-right screen margins.
// Physical screen pixels; divided by DPI before writing UMG translations.
const EXTRA_CORNER_X_PX: f64 = 18.0;
const EXTRA_CORNER_Y_PX: f64 = 18.0;

thread_local! {
    static REPORTED: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    static ORIGINAL_X: RefCell<HashMap<String, OffsetState>> = RefCell::new(HashMap::new());
    static ORIGINAL_Y: RefCell<HashMap<String, OffsetState>> = RefCell::new(HashMap::new());
}

/// Only trust a fresh viewport measurement. The SDK returns 1x1 if no world
/// resolves; reusing cached viewport data during a resolution change would
/// risk applying the OLD wide offset at 16:9. Find an attached ancestor instead.
fn viewport_size_and_scale(widget: *mut UWidget) -> Option<(f64, f64, f64)> {
    let mut node = widget;
    for _ in 0..32 {
        if !live(node) {
            return None;
        }
        let size = UWidgetLayoutLibrary::GetViewportSize(node.cast());
        let scale = f64::from(UWidgetLayoutLibrary::GetViewportScale(node.cast()));
        if size.X.is_finite()
            && size.Y.is_finite()
            && size.X > 8.0
            && size.Y > 8.0
            && scale.is_finite()
            && scale > 0.0
        {
            return Some((size.X, size.Y, scale));
        }
        node = outward(node);
    }
    None
}

fn report_once(key: String, message: String) {
    REPORTED.with_borrow_mut(|reported| {
        if reported.len() < 240 && reported.insert(key) {
            log::info!("{}", message);
        }
    });
}

/// Runs on a class-specific widget. The transform path writes the field in
/// place (the native engine is about to consume it), whereas the rebuild path
/// calls the setter to notify Slate. All writes are opt-in and exact-class.
unsafe fn consider(widget: *mut UWidget, probe: bool, enabled: bool, hint: &str, rebuild: bool) {
    if (!probe && !enabled) || !live(widget) {
        return;
    }
    unsafe {
        if !(*widget).IsA(UUserWidget::StaticClass()) {
            return;
        }
        let class = (&*(&(*widget)).Class).GetName();
        if class != MINIMAP_LAYOUT_CLASS {
            return;
        }
        let name = (*widget).GetName();
        let id = format!("{}:{}:{}", class, name, (&(*widget)).Index);
        let phase = if rebuild { "rebuild" } else { "transform" };
        let Some((width, height, dpi)) = viewport_size_and_scale(widget) else {
            if probe {
                report_once(
                    format!("pending:{id}:{phase}"),
                    format!("MINIMAP_V3_1_TARGET phase={} id={} viewport=unavailable (no edit)", phase, id),
                );
            }
            return;
        };
        let ratio = width / height;
        // Tolerance also absorbs tiny rounding differences at exact 16:9.
        // Wide screens: restore the root's 16:9 side gutter, then nudge the
        // minimap nearer to the physical top-right edge. Exact 16:9 stays 0,0.
        let (shift_x, shift_y) = if ratio > BASE_ASPECT + 0.001 {
            (
                (width - height * BASE_ASPECT).max(0.0) / (2.0 * dpi)
                    + EXTRA_CORNER_X_PX / dpi,
                -EXTRA_CORNER_Y_PX / dpi,
            )
        } else {
            (0.0, 0.0)
        };
        if probe {
            report_once(
                format!("target:{id}:{phase}:{}", shift_x.round()),
                format!(
                    "MINIMAP_V3_1_TARGET phase={} id={} viewport={}x{} dpi={} shift_local={:.2},{:.2}",
                    phase, id, width, height, dpi, shift_x, shift_y,
                ),
            );
        }

        // Deliberately require an explicit, exact class name in the TOML.
        if !enabled || hint.trim() != MINIMAP_LAYOUT_CLASS {
            return;
        }

        // At 16:9, do not alter a newly created widget. An instance that
        // was previously shifted can be restored in either axis instead.
        let current_x = (*widget).RenderTransform.Translation.X as f32;
        let current_y = (*widget).RenderTransform.Translation.Y as f32;
        let next_x = ORIGINAL_X.with_borrow_mut(|states| {
            if shift_x == 0.0 && !states.contains_key(&id) {
                return None;
            }
            let state = states.entry(id.clone()).or_default();
            let shifted = apply_offset(state, current_x, shift_x as f32);
            if (shifted - current_x).abs() < 0.0001 { None } else { Some(shifted) }
        });
        let next_y = ORIGINAL_Y.with_borrow_mut(|states| {
            if shift_y == 0.0 && !states.contains_key(&id) {
                return None;
            }
            let state = states.entry(id.clone()).or_default();
            let shifted = apply_offset(state, current_y, shift_y as f32);
            if (shifted - current_y).abs() < 0.0001 { None } else { Some(shifted) }
        });
        if next_x.is_none() && next_y.is_none() {
            return;
        }
        let applied_x = next_x.unwrap_or(current_x);
        let applied_y = next_y.unwrap_or(current_y);
        if rebuild {
            // No RefCell borrow survives this call; it can re-enter callbacks.
            let translation = FVector2D::new(f64::from(applied_x), f64::from(applied_y));
            (*widget).SetRenderTranslation(&translation);
        } else {
            (*widget).RenderTransform.Translation.X = f64::from(applied_x);
            (*widget).RenderTransform.Translation.Y = f64::from(applied_y);
        }
        report_once(
            format!("move31:{id}:{}:{}", shift_x.round(), shift_y.round()),
            format!(
                "MINIMAP_V3_1_{} phase={} id={} from=({:.2},{:.2}) to=({:.2},{:.2}) shift=({:.2},{:.2})",
                if shift_x == 0.0 && shift_y == 0.0 { "RESTORED" } else { "MOVED" },
                phase, id, current_x, current_y, applied_x, applied_y, shift_x, shift_y,
            ),
        );
    }
}

/// Called from the existing UWidget UpdateRenderTransform hook.
pub(super) unsafe fn on_transform(widget: *mut UWidget, probe: bool, enabled: bool, hint: &str) {
    unsafe { consider(widget, probe, enabled, hint, false) }
}

/// Called from the existing UUserWidget rebuild hook.
pub(super) unsafe fn on_rebuild(widget: *mut UWidget, probe: bool, enabled: bool, hint: &str) {
    unsafe { consider(widget, probe, enabled, hint, true) }
}
