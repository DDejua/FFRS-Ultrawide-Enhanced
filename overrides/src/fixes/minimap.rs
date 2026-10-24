//! v3.6 candidate - lean HUD menu transitions (TEST ONLY).
//! Only the outer WBP_MiniMapLayout_C is affected; descendants follow it.
//! Never change a new widget at 16:9. A previously shifted instance is
//! restored to its original translation when a later callback sees 16:9.

use std::{cell::{Cell, RefCell}, collections::{HashMap, HashSet}, time::{Duration, Instant}};

use sdk::{
    CoreUObject::{FVector2D, UObject},
    UMG::{ESlateVisibility, UUserWidget, UWidget, UWidgetLayoutLibrary},
    rt::NameCache,
};

use super::{
    layout::{OffsetState, apply_offset},
    widgets::{live, outward},
};

const MINIMAP_LAYOUT_CLASS: &str = "WBP_MiniMapLayout_C";
const FIELD_QUEST_CLASS: &str = "WBP_FieldQuestLayout_C";
// Deliberately small, diagnostic-only grace period after a menu collapses.
// Prevents early minimap/quest reveal if another menu opens immediately.
const MENU_CLOSE_GRACE: Duration = Duration::from_millis(160);
const BASE_ASPECT: f64 = 16.0 / 9.0;
// Additional space taken from the author's top-right screen margins.
// Physical screen pixels; divided by DPI before writing UMG translations.
const EXTRA_CORNER_X_PX: f64 = 18.0;
const EXTRA_CORNER_Y_PX: f64 = 18.0;

thread_local! {
    static REPORTED: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    static ORIGINAL_X: RefCell<HashMap<String, OffsetState>> = RefCell::new(HashMap::new());
    static ORIGINAL_Y: RefCell<HashMap<String, OffsetState>> = RefCell::new(HashMap::new());
    static MENU_ACTIVE: Cell<bool> = const { Cell::new(false) };
    static MENU_CLOSE_PENDING: Cell<Option<Instant>> = const { Cell::new(None) };
    static GAME_TICK_SEEN: Cell<bool> = const { Cell::new(false) };
    // Store an object index and full identity instead of a stale raw pointer.
    static ACTIVE_MINIMAP: RefCell<Option<(i32, String)>> = const { RefCell::new(None) };
    static ACTIVE_QUEST: RefCell<Option<(i32, String)>> = const { RefCell::new(None) };
    static MENU_OPACITY: RefCell<HashMap<String, f32>> = RefCell::new(HashMap::new());
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
        static MINIMAP_CLASS: NameCache = NameCache::new("WBP_MiniMapLayout_C");
        if (&*(&(*widget)).Class).Name != MINIMAP_CLASS.get() {
            return;
        }
        let name = (*widget).GetName();
        let id = format!("{}:{}:{}", MINIMAP_LAYOUT_CLASS, name, (&(*widget)).Index);
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
        // Wide screens outside a menu: restore the root's 16:9 side gutter, then nudge the
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

        // Remember the confirmed live instance for immediate adjustment when
        // Stack_System changes. GObjects is re-queried on every menu event.
        let full_name = (*widget).GetFullName();
        ACTIVE_MINIMAP.with_borrow_mut(|target| {
            *target = Some(((&(*widget)).Index, full_name));
        });

        // At 16:9 or while a menu is active, do not alter a newly created widget. An instance that
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
    if enabled { unsafe { maybe_finish_menu_close(); } }
    if enabled { unsafe { observe_quest(widget); } }
    unsafe { consider(widget, probe, enabled, hint, false) }
}

/// Called from the existing UUserWidget rebuild hook.
pub(super) unsafe fn on_rebuild(widget: *mut UWidget, probe: bool, enabled: bool, hint: &str) {
    if enabled { unsafe { maybe_finish_menu_close(); } }
    if enabled { unsafe { observe_quest(widget); } }
    unsafe { consider(widget, probe, enabled, hint, true) }
}

///// Diagnostic-only fix candidate. Called from the existing SetVisibility hook.
/// Treat only Stack_System's displayed/collapsed transitions as menu changes.
/// Never manipulate the stack itself or affect the original menu widgets.
///
/// We re-resolve the minimap by GObjects index and compare its full name,
/// avoiding cached pointers across gameplay/world transitions.
pub(super) unsafe fn on_menu_visibility(
    widget: *mut UWidget,
    requested: ESlateVisibility,
    enabled: bool,
    hint: &str,
) {
    if !enabled || hint.trim() != MINIMAP_LAYOUT_CLASS || !live(widget) {
        return;
    }
    unsafe {
        static STACK_CLASS: NameCache = NameCache::new("CPP_EU_ActivatableWidgetStack");
        static STACK_NAME: NameCache = NameCache::new("Stack_System");
        if (&*(&(*widget)).Class).Name != STACK_CLASS.get()
            || (*widget).Name != STACK_NAME.get()
        {
            return;
        }
    }
    let active = if requested == ESlateVisibility::SelfHitTestInvisible
        || requested == ESlateVisibility::Visible
    {
        true
    } else if requested == ESlateVisibility::Collapsed
        || requested == ESlateVisibility::Hidden
    {
        false
    } else {
        return;
    };
    if MENU_ACTIVE.replace(active) == active {
        return;
    }
    log::info!("MINIMAP_MENU_GATE active={} stack=Stack_System", active);

    if !active {
        // Stack_System may disappear BEFORE its visual exit animation is
        // finished; keep both overlays hidden until the grace interval.
        MENU_CLOSE_PENDING.set(Some(Instant::now()));
        log::info!("HUD_OPACITY_GRACE pending={}ms", MENU_CLOSE_GRACE.as_millis());
        return;
    }
    // Reopening during a pending close cancels any deferred reveal.
    MENU_CLOSE_PENDING.set(None);
    unsafe { apply_known_menu_opacity(true); }
}


/// TEST ONLY: observe the fixed mission-objective overlay. This specific class
/// appears in the on-disk UI diagnostic log on field entry. If it is not the
/// correct owner, the debug build will show no improvement to the left line.
pub(super) unsafe fn on_widget_visibility(widget: *mut UWidget, enabled: bool) {
    if enabled { unsafe { maybe_finish_menu_close(); } }
    if !enabled { return; }
    unsafe { observe_quest(widget); }
}

unsafe fn observe_quest(widget: *mut UWidget) {
    if !live(widget) { return; }
    static QUEST_CLASS: NameCache = NameCache::new("WBP_FieldQuestLayout_C");
    if unsafe { (&*(&(*widget)).Class).Name } != QUEST_CLASS.get() { return; }
    let full_name = unsafe { (*widget).GetFullName() };
    let index = unsafe { (&(*widget)).Index };
    let was_changed = ACTIVE_QUEST.with_borrow_mut(|entry| {
        if entry.as_ref().is_some_and(|v| v.0 == index && v.1 == full_name) {
            false
        } else {
            *entry = Some((index, full_name.clone()));
            true
        }
    });
    if was_changed {
        let mut ancestry = Vec::new();
        let mut cur = widget;
        for _ in 0..10 {
            let outer = outward(cur);
            if !live(outer) { break; }
            ancestry.push(unsafe { (&*(&(*outer)).Class).GetName() });
            cur = outer;
        }
        log::info!("HUD_OPACITY_QUEST_TRACK index={} parents={:?}", index, ancestry);
    }
    if MENU_ACTIVE.get() {
        unsafe { set_menu_opacity(widget, true, "field_quest"); }
    }
}

/// Stores the original opacity on entry, restores it on exit. It never writes
/// Visibility or relocates the widget, so the game's hit-testing/layout is
/// unchanged. All operations run on the UMG game thread.
unsafe fn set_menu_opacity(widget: *mut UWidget, menu_active: bool, label: &str) {
    if !live(widget) { return; }
    let key = unsafe { (*widget).GetFullName() };
    let current = unsafe { (*widget).RenderOpacity };
    if menu_active {
        MENU_OPACITY.with_borrow_mut(|map| {
            if !map.contains_key(&key) {
                map.insert(key.clone(), current);
            }
        });
        if current > 0.0001 {
            unsafe { (*widget).SetRenderOpacity(0.0); }
            log::info!("HUD_OPACITY_GUARD hide={} from={:.3}", label, current);
        }
    } else {
        let baseline = MENU_OPACITY.with_borrow_mut(|map| map.remove(&key));
        if let Some(opacity) = baseline {
            // Avoid redundant setter calls.
            if (current - opacity).abs() > 0.0001 {
                unsafe { (*widget).SetRenderOpacity(opacity); }
                log::info!("HUD_OPACITY_GUARD show={} to={:.3}", label, opacity);
            }
        }
    }
}

/// Called by the game-thread FEngineLoop tick and by existing UMG hooks as
/// fallback. No threads, sleeping, new timers or out-of-thread UMG writes.
unsafe fn maybe_finish_menu_close() {
    if MENU_ACTIVE.get() { return; }
    let Some(closed_at) = MENU_CLOSE_PENDING.get() else { return; };
    if closed_at.elapsed() < MENU_CLOSE_GRACE { return; }
    MENU_CLOSE_PENDING.set(None);
    unsafe { apply_known_menu_opacity(false); }
    log::info!("HUD_OPACITY_GRACE elapsed={}ms restored=true", closed_at.elapsed().as_millis());
}

/// Independently validates cached minimap and quest identities. If the
/// minimap has been collected, the quest overlay is still processed.
unsafe fn apply_known_menu_opacity(active: bool) {
    let target = ACTIVE_MINIMAP.with_borrow(|v| v.clone());
    if let Some((index, full_name)) = target {
        let object = UObject::GObjects.GetByIndex(index);
        if live(object) && unsafe { (*object).GetFullName() } == full_name {
            let minimap = object.cast::<UWidget>();
            if unsafe { (*minimap).IsA(UUserWidget::StaticClass()) }
                && unsafe { (&*(&(*minimap)).Class).Name }
                    == NameCache::new("WBP_MiniMapLayout_C").get()
            {
                unsafe { set_menu_opacity(minimap, active, "minimap"); }
            } else {
                ACTIVE_MINIMAP.with_borrow_mut(|v| *v = None);
                log::info!("HUD_OPACITY_GUARD minimap=unexpected_class");
            }
        } else {
            ACTIVE_MINIMAP.with_borrow_mut(|v| *v = None);
            log::info!("HUD_OPACITY_GUARD minimap=stale");
        }
    } else {
        log::info!("HUD_OPACITY_GUARD minimap=not_found");
    }
    let quest = ACTIVE_QUEST.with_borrow(|v| v.clone());
    if let Some((index, full_name)) = quest {
        let obj = UObject::GObjects.GetByIndex(index);
        static QUEST_CLASS: NameCache = NameCache::new("WBP_FieldQuestLayout_C");
        if live(obj) && unsafe { (*obj).GetFullName() } == full_name
            && unsafe { (&*(&(*obj)).Class).Name } == QUEST_CLASS.get()
        {
            unsafe { set_menu_opacity(obj.cast(), active, "field_quest"); }
        } else {
            ACTIVE_QUEST.with_borrow_mut(|v| *v = None);
            log::info!("HUD_OPACITY_GUARD quest=stale");
        }
    } else {
        log::info!("HUD_OPACITY_GUARD quest=not_found");
    }
}


/// EXPERIMENTAL: Called once per engine frame from a pinned native signature.
/// This is the SAME game thread the widget hooks use; no worker thread is
/// allowed to touch UMG. Runs when stationary, paused or simply looking at
/// the menu (as long as FEngineLoop continues to tick).
pub(super) fn on_game_thread_tick() {
    if !GAME_TICK_SEEN.replace(true) {
        log::info!("HUD_OPACITY_FRAME_TICK first_tick=true");
    }
    unsafe { maybe_finish_menu_close(); }
}
