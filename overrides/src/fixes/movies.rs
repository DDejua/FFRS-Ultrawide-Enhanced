//! Everything that follows a playing movie: fitting its image to 16:9 and the
//! black backdrop wrapped behind it.

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ptr::null_mut,
};

use sdk::{
    CoreUObject::{FVector2D, UObject},
    UMG::{ESlateVisibility, UOverlay, UUserWidget, UWidget},
    mucha::{
        EeBtlSequenceEventType, FBTL_SEQUENCE_PARAMETER, UCPP_WBP_MovieProjector,
        UCPP_WBP_MovieScreen,
    },
    rt::NameCache,
};

use super::{
    layout::{Scale, fit_16_9},
    widgets::{
        BackdropWrapper, WrapGuard, apply_center_scale, approximately, arranged, describe,
        expand_backdrop, fill_overlay, layout_box, live, live_backdrop, mirrored_visibility, spawn,
        spawn_backdrop_image,
    },
};
use crate::utils::{ModuleInfo, SignatureHook, inject_hook, log_report};

static MOVIE_NAME: NameCache = NameCache::new("Image_Movie");
static BATTLE_CINEMATIC: NameCache = NameCache::new("WBP_BtlCinematicLayout_C");

thread_local! {
    /// Every movie root this mod has wrapped, by the root's full name.
    static WRAPPED_ROOTS: RefCell<HashMap<String, BackdropWrapper>> = RefCell::new(HashMap::new());

    /// Set while [`on_rebuild`] spawns widgets, so the visibility changes that
    /// spawning triggers are not mistaken for the game dismissing a movie.
    static WRAPPING: Cell<bool> = const { Cell::new(false) };

    /// Whether a sequence has put a movie on screen, as the sequence itself says.
    ///
    /// A summon authors UISetVisibility with its flag clear when its cinematic
    /// takes the whole screen, and a sequence that names its movie directly
    /// authors UISetVisibilityMovie. The later event wins, which is the
    /// sequence's latest instruction either way.
    static MOVIE_PHASE: Cell<bool> = const { Cell::new(false) };
}

/// Fits a playing movie to 16:9 from the native callbacks that own its image:
/// the movie screen's frame update and texture assignment, and the point where
/// creating a movie screen completes.
///
/// The Projector field at +0x368, which `on_native_movie` reads through the
/// SDK, stays literal in the first two, as does the RBX the completion hook
/// reads; frame sizes, branches and moved-through registers are wildcarded.
pub(super) fn install(module: &ModuleInfo) {
    let frame = SignatureHook {
        tag: "MovieScreen::OnFrame",
        signature: "40 53 48 83 EC ?? 48 8B 99 68 03 00 00 48 8B 8B 68 03 00 00 E8 ?? ?? ?? ?? 48 85 C0 74 ??",
        offset: 0,
    };
    inject_hook(module, &frame, |ctx| {
        on_native_movie(ctx.rcx as *mut UObject, true);
    });
    let texture = SignatureHook {
        tag: "MovieProjector::SetManaTexture",
        signature: "48 89 5C 24 10 57 48 83 EC ?? 48 8B ?? 48 8B ?? 48 8B 89 68 03 00 00 E8 ?? ?? ?? ??",
        offset: 0,
    };
    inject_hook(module, &texture, |ctx| {
        // RDX is the incoming texture; a cleared one starts no playback.
        if ctx.rdx == 0 {
            return;
        }
        on_native_movie(ctx.rcx as *mut UObject, false);
    });
    let created = SignatureHook {
        tag: "CreateMovieScreen completion",
        signature: "33 D2 48 89 5F ?? 48 8B CB E8 ?? ?? ?? ?? 48 8B 47 ?? 48 83 C4 ?? 5F 5E 5B C3",
        offset: 14,
    };
    inject_hook(module, &created, |ctx| {
        // RBX holds the screen the creation stored, null when it failed.
        if ctx.rbx == 0 {
            return;
        }
        on_native_movie(ctx.rbx as *mut UObject, true);
    });
    // A battle sequence dispatches its authored events through one native
    // function, tagged by the EventType byte at the head of the parameter
    // struct. The signature is that dispatch reading the tag and bounding it
    // against the last event type: MOVZX EAX,byte ptr [R8]; DEC EAX;
    // CMP EAX,0x7A. The bound stays literal: wildcarded, it also matches a
    // nine-case dispatch with the same read. Hooking the read rather than the
    // function entry keeps the anchor adjacent to the field that is read, and
    // R8 still holds the struct there.
    let sequence = SignatureHook {
        tag: "Battle sequence event",
        signature: "41 0F B6 00 FF C8 83 F8 7A 0F 87 ?? ?? ?? ??",
        offset: 0,
    };
    inject_hook(module, &sequence, |ctx| {
        on_battle_sequence_event(ctx.r8 as *const FBTL_SEQUENCE_PARAMETER);
    });
}

/// Whether a wrapped root is a cinematic's movie image.
///
/// A cinematic's movie image shares its layout with an in-engine lead-in, so
/// its backdrop waits for the sequence. A movie screen or movie widget shows
/// nothing but a movie, and keeps following its own visibility.
fn waits_for_sequence(root: *const UWidget) -> bool {
    !root.is_null() && unsafe { &*root }.Name == MOVIE_NAME.get()
}

/// Whether the black image behind a movie should be drawn yet.
///
/// The gate belongs on the backdrop alone. The wrapper it hangs in is the
/// cinematic's widget-tree root and holds the movie image as its other child,
/// so hiding the wrapper takes the movie down with the backdrop and the
/// cinematic shows nothing at all. Hidden rather than Collapsed keeps the
/// image arranged, so the geometry the expansion measures stays available and
/// the bars are already the right size when they are drawn.
fn backdrop_visibility(root: *const UWidget) -> ESlateVisibility {
    if waits_for_sequence(root) && !MOVIE_PHASE.get() {
        ESlateVisibility::Hidden
    } else {
        ESlateVisibility::HitTestInvisible
    }
}

/// Geometry for two solid-black side strips, leaving the central movie and
/// its transparent pixels untouched. All measurements use local UMG units.
/// Does not assume a 21:9 (or 32:9) resolution or a particular HUD preset.
#[derive(Debug, PartialEq)]
struct SideStrips {
    width: f64,
    offset: f64,
    scale: Scale,
}

fn side_strips(
    box_width: f64,
    box_height: f64,
    movie_width: f64,
    movie_height: f64,
    screen_width: f64,
    screen_height: f64,
) -> Option<SideStrips> {
    let values = [
        box_width,
        box_height,
        movie_width,
        movie_height,
        screen_width,
        screen_height,
    ];
    if values.iter().any(|n| !n.is_finite() || *n < 1.0) {
        return None;
    }
    // The existing Image_Movie correction fits the same 16:9 width.
    let fitted_width = movie_width * fit_16_9(movie_width, movie_height).x;
    let margin = (screen_width - fitted_width) * 0.5;
    if margin < 0.5 {
        return None;
    }
    Some(SideStrips {
        width: margin,
        offset: (fitted_width + margin) * 0.5,
        scale: Scale {
            x: margin / box_width,
            y: screen_height / box_height,
        },
    })
}

/// Position a mod-owned black image outside the picture, around its centered
/// pivot. Existing masks extend beyond a centered HUD's box using Slate's
/// default unclipped render transform, just like the prior full black image.
fn position_side_strip(image: *mut UWidget, scale: Scale, translation_x: f64) -> bool {
    let resized = apply_center_scale(image, scale);
    unsafe {
        let current = (*image).RenderTransform.Translation;
        let moved = !approximately(current.X, translation_x) || !approximately(current.Y, 0.0);
        if moved {
            (*image).SetRenderTranslation(&FVector2D::new(translation_x, 0.0));
        }
        resized || moved
    }
}

/// Re-gates and positions backdrops for every wrapped movie.
///
/// Story/general Projector trees retain their original full black background.
/// Only Image_Movie owned by WBP_BtlCinematicLayout_C has three overlay
/// children: LEFT STRIP, RIGHT STRIP, ORIGINAL MOVIE. The strips cover solely
/// the viewport regions *outside* the fitted 16:9 picture; the center can still
/// reveal the 3D battle through the movie's authored transparency.
///
/// The existing MOVIE_PHASE gate remains unchanged for this experiment. This
/// isolates the geometry/layering change from the phase-detection heuristic.
/// Collect indices out of the registry first because widget setters reenter
/// the same UMG hooks.
fn refresh_movie_backdrops() {
    let wrappers: Vec<_> = WRAPPED_ROOTS.with_borrow(|roots| {
        roots
            .iter()
            .map(|(name, wrapper)| (name.clone(), wrapper.clone()))
            .collect()
    });
    for (root_name, wrapper) in wrappers {
        let black = live_backdrop(&wrapper);
        if black.is_null() {
            continue;
        }
        unsafe {
            let image = &mut *black;
            let overlay = if live(image.Slot) {
                (*image.Slot).Parent
            } else {
                null_mut()
            };
            if !live(overlay) {
                continue;
            }
            let children = (*overlay).GetChildrenCount();
            if children < 2 {
                continue;
            }
            let root = (*overlay).GetChildAt(children - 1);
            if !live(root) {
                continue;
            }
            let value = backdrop_visibility(root);
            if children == 3 && waits_for_sequence(root) {
                // Child 1 is the second mod-owned UImage, created only for the
                // verified WBP_BtlCinematicLayout_C.Image_Movie root.
                let right = (*overlay).GetChildAt(1);
                if !live(right) {
                    continue;
                }
                let left_layout = layout_box(black.cast());
                let movie_layout = layout_box(root);
                let geometry = side_strips(
                    left_layout.size.X,
                    left_layout.size.Y,
                    movie_layout.size.X,
                    movie_layout.size.Y,
                    left_layout.screen.X,
                    left_layout.screen.Y,
                );
                // At native 16:9 there is no strip to paint. Skip invalid
                // transient geometry rather than covering the picture.
                let effective_visibility = if geometry.is_some() {
                    value
                } else {
                    ESlateVisibility::Hidden
                };
                for mask in [black.cast::<UWidget>(), right] {
                    if (*mask).Visibility != effective_visibility {
                        (*mask).SetVisibility(effective_visibility);
                    }
                }
                if let Some(strips) = geometry {
                    let left_changed = position_side_strip(
                        black.cast(),
                        strips.scale,
                        -strips.offset,
                    );
                    let right_changed = position_side_strip(right, strips.scale, strips.offset);
                    if left_changed || right_changed {
                        log_report(
                            &format!("battle_movie_strips:{root_name}"),
                            format!(
                                "Battle movie side strips: {root_name}, width={} per side, offset={}, box={}x{}, screen={}x{}",
                                strips.width,
                                strips.offset,
                                left_layout.size.X,
                                left_layout.size.Y,
                                left_layout.screen.X,
                                left_layout.screen.Y
                            ),
                        );
                    }
                }
                continue;
            }

            // Original upstream handling, untouched for story/general FMVs.
            if image.Visibility != value {
                image.SetVisibility(value);
                log_report(
                    &format!("gate:{root_name}"),
                    format!(
                        "Movie backdrop gate: {root_name}, drawn={}",
                        value == ESlateVisibility::HitTestInvisible
                    ),
                );
            }
        }
        expand_backdrop(
            black.cast(),
            &format!("backdrop:{root_name}"),
            &format!("Movie backdrop: {root_name}"),
        );
    }
}

/// The widget-tree root to wrap, or null when this is not a movie or cannot be
/// wrapped.
fn wrappable_movie_root(user: *mut UUserWidget) -> *mut UWidget {
    unsafe {
        let tree = (*user).WidgetTree;
        if tree.is_null() || (*tree).RootWidget.is_null() {
            return null_mut();
        }
        let root = (*tree).RootWidget;
        let name = (*root).GetName();
        // Only wrap a widget-tree root, never reparent a content child.
        if (name != "Image_Movie" && name != "Projector") || !(*root).Slot.is_null() {
            return null_mut();
        }
        root
    }
}

/// Carries a battle sequence's own decisions to the movie backdrop.
///
/// A summon builds its cinematic and makes the movie image Visible when the
/// action starts, then plays an in-engine animation over the battle before the
/// movie itself appears, so neither the widget's visibility nor Mana's playback
/// state marks the cut: a capture of a real summon has visibility, both
/// bUseMovie flags and a Mana component all arriving together at the start of
/// the lead-in. The sequence authors its own events instead.
/// FBTL_SEQUENCE_PARAMETER is a tagged union whose EventType selects which of
/// its fields apply.
///
/// A run of this hook shows the summon authoring UISetVisibility with its flag
/// clear 1.07s after the cinematic was wrapped, and no UISetVisibilityMovie at
/// all, so the battle UI being hidden is what marks the movie phase: the
/// lead-in is a camera move over a battle whose HUD is still up, and hiding
/// that HUD is the cinematic taking the whole screen. A sequence that names
/// its movie directly is honoured through the other event.
///
/// `event` is the FBTL_SEQUENCE_PARAMETER, from R8 where the dispatch reads its
/// EventType.
fn on_battle_sequence_event(event: *const FBTL_SEQUENCE_PARAMETER) {
    if event.is_null() {
        return;
    }
    let event = unsafe { &*event };
    let named = match event.EventType {
        EeBtlSequenceEventType::UISetVisibilityMovie => {
            MOVIE_PHASE.set(event.UI_SetVisibillityMovie_IsVisibility);
            "movie visibility"
        }
        EeBtlSequenceEventType::UISetVisibility => {
            MOVIE_PHASE.set(!event.UI_SetVisibillity_IsVisibility);
            "battle UI hidden"
        }
        _ => return,
    };
    let phase = if MOVIE_PHASE.get() { "on" } else { "off" };
    log_report("movie_phase", format!("Battle sequence: {named} {phase}"));
    // The event names no widget, so every wrapper is asked again.
    refresh_movie_backdrops();
}

/// Letterboxes a summon's movie image to 16:9, from the shared render-transform
/// hook that is about to consume the scale field.
///
/// The image is fitted to its own arranged box, not to the viewport: an
/// ancestor held inside 16:9 already hands it a 16:9 box, and scaling that by
/// the viewport's wider aspect would letterbox a second time.
///
/// # Safety
///
/// `widget` is the widget in RCX at native UpdateRenderTransform, on the game
/// thread.
pub(super) unsafe fn on_transform(widget: *mut UWidget) {
    if unsafe { &*widget }.Name != MOVIE_NAME.get() {
        return;
    }
    let layout = layout_box(widget);
    if !arranged(&layout.size) {
        return;
    }
    let fit = fit_16_9(layout.size.X, layout.size.Y);
    let next = FVector2D::new(fit.x, fit.y);
    let scale = unsafe { &mut (*widget).RenderTransform.Scale };
    let refit = !approximately(scale.X, next.X) || !approximately(scale.Y, next.Y);
    // Native UpdateRenderTransform consumes this field immediately after the
    // hook, so it is written every time even when the value is unchanged.
    *scale = next;
    if refit {
        log_report(
            "summon_movie",
            format!("Native summon image: {}", describe(&layout, fit)),
        );
    }
    refresh_movie_backdrops();
}

/// Letterboxes a playing FMV to 16:9.
///
/// Invoked by the native movie frame and texture-assignment hooks, never by a
/// timer, so the fit is re-applied exactly when the engine touches the image.
/// `object` is a `UCPP_WBP_MovieScreen` when `is_screen`, else a
/// `UCPP_WBP_MovieProjector`.
fn on_native_movie(object: *mut UObject, is_screen: bool) {
    unsafe {
        let projector = if is_screen {
            (*object.cast::<UCPP_WBP_MovieScreen>()).Projector
        } else {
            object.cast::<UCPP_WBP_MovieProjector>()
        };
        if projector.is_null() || (*projector).Projector.is_null() {
            return;
        }
        let picture = (*projector).Projector.cast::<UWidget>();
        let layout = layout_box(picture);
        if !arranged(&layout.size) {
            return;
        }
        let fit = fit_16_9(layout.size.X, layout.size.Y);
        // GetFullName walks the outer chain building a string, so the key is
        // only built on a frame that actually refits.
        if apply_center_scale(picture, fit) {
            let label = if is_screen {
                "OnFrame"
            } else {
                "SetManaTexture"
            };
            log_report(
                &format!("native_movie:{}", (*projector).GetFullName()),
                format!("Native FMV: {label}, {}", describe(&layout, fit)),
            );
        }
    }
    refresh_movie_backdrops();
}

/// Carries a wrapped movie root's visibility onto the wrapper that holds its
/// black backdrop, so the backdrop is dismissed with the movie.
///
/// A Visible movie becomes SelfHitTestInvisible on the wrapper so the overlay
/// never takes input the movie itself would not have taken.
///
/// # Safety
///
/// `widget` is the widget in RCX and `visibility` the new value in DL at native
/// `UWidget::SetVisibility`, on the game thread.
pub(super) unsafe fn on_visibility(widget: *mut UWidget, visibility: ESlateVisibility) {
    if WRAPPING.get() {
        return;
    }
    unsafe {
        let slot = (*widget).Slot;
        if slot.is_null() || (*slot).Parent.is_null() {
            return;
        }
        let parent = &mut *(*slot).Parent;
        let name = (&*widget).GetFullName();
        let wrapped = WRAPPED_ROOTS
            .with_borrow(|roots| roots.get(&name).is_some_and(|w| w.overlay == parent.Index));
        if !wrapped {
            return;
        }
        let value = mirrored_visibility(visibility);
        if parent.Visibility != value {
            parent.SetVisibility(value);
        }
    }
}

/// Wraps a movie's widget-tree root in an overlay carrying a black backdrop.
///
/// Runs from the user-widget rebuild, before the tree root is consumed for the
/// Slate rebuild, which is the one moment the root can be replaced. The overlay
/// holds the black image behind the movie and the movie above it, both filling
/// the overlay, so the bars beside a letterboxed movie are black rather than
/// the world behind it.
///
/// # Safety
///
/// `user` is the UUserWidget in RBX at the rebuild, on the game thread.
pub(super) unsafe fn on_rebuild(user: *mut UUserWidget) {
    if WRAPPING.get() {
        return;
    }
    let root = wrappable_movie_root(user);
    if root.is_null() {
        return;
    }

    let _guard = WrapGuard::new(&WRAPPING);

    unsafe {
        let tree = (*user).WidgetTree;
        let overlay = spawn::<UOverlay>(tree);
        let black = spawn_backdrop_image(tree);
        // Only the observed battle-cinematic class gets two side strips.
        // Projector (including story FMVs) retains its original one-image
        // backdrop and both native 16:9 fitting paths remain unchanged.
        let battle = waits_for_sequence(root)
            && (&*(*user).Class).Name == BATTLE_CINEMATIC.get();
        let right = if battle { spawn_backdrop_image(tree) } else { null_mut() };
        if overlay.is_null() || black.is_null() || (battle && right.is_null()) {
            log::warn!("Movie backdrop: allocation failed.");
            return;
        }
        if !fill_overlay(overlay, black.cast()) {
            log::warn!("Movie backdrop: background slot failed.");
            return;
        }
        if battle && !fill_overlay(overlay, right.cast()) {
            log::warn!("Movie backdrop: second side strip slot failed.");
            return;
        }
        if !fill_overlay(overlay, root) {
            log::warn!("Movie backdrop: movie slot failed.");
            return;
        }

        if waits_for_sequence(root) {
            // A new cinematic begins with its lead-in, whatever the last one said.
            MOVIE_PHASE.set(false);
        }
        let gate = backdrop_visibility(root);
        (*black).SetVisibility(if battle { ESlateVisibility::Hidden } else { gate });
        if battle {
            (*right).SetVisibility(ESlateVisibility::Hidden);
        }
        (*overlay).SetVisibility(mirrored_visibility((*root).Visibility));
        (*tree).RootWidget = overlay.cast();
        let black = &*black;
        let wrapper = BackdropWrapper {
            overlay: (&*overlay).Index,
            black: black.Index,
            black_name: black.GetFullName(),
        };
        let root_name = (&*root).GetFullName();
        WRAPPED_ROOTS.with_borrow_mut(|roots| roots.insert(root_name, wrapper));
        log::info!(
            "Native movie backdrop: wrapped {}.{}, battle_side_strips={battle}",
            (*user).GetName(),
            (*root).GetName()
        );
    }
}

#[cfg(test)]
mod side_strip_tests {
    use super::side_strips;

    #[test]
    fn center_stays_transparent_and_bars_are_geometry_derived() {
        let ultrawide = side_strips(1920.0, 1080.0, 1920.0, 1080.0, 2580.0, 1080.0).unwrap();
        assert_eq!(ultrawide.width, 330.0);
        assert_eq!(ultrawide.offset, 1125.0);
        assert_eq!(ultrawide.scale.x, 330.0 / 1920.0);
        // 5120x1440, with viewport scale 1.333..., corresponds to
        // 3840x1080 UMG local units.
        let superwide = side_strips(1920.0, 1080.0, 1920.0, 1080.0, 3840.0, 1080.0).unwrap();
        assert_eq!(superwide.width, 960.0);
        assert_eq!(superwide.offset, 1440.0);
        // With the ULTRAWIDE HUD preset, movie content is still fit to 16:9.
        let wide_hud = side_strips(2520.0, 1080.0, 2520.0, 1080.0, 2580.0, 1080.0).unwrap();
        assert_eq!(wide_hud.width, 330.0);
        assert_eq!(wide_hud.offset, 1125.0);
    }

    #[test]
    fn no_side_bars_on_native_16_9_or_invalid_geometry() {
        assert!(side_strips(1920.0, 1080.0, 1920.0, 1080.0, 1920.0, 1080.0).is_none());
        assert!(side_strips(1920.0, 1080.0, 1920.0, 1080.0, 0.0, 1080.0).is_none());
    }
}
