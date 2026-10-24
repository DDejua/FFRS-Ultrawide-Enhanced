//! One module per fix: [`aspect_ratio`] and [`background`] patch bytes, [`hud`]
//! owns everything that follows the centered UI root, [`map`] the world map's
//! backdrop, [`movies`] everything that follows a playing movie, [`encounter`]
//! the screen capture an encounter transition draws. [`layout`] and [`widgets`]
//! hold what they share.
//!
//! A mid-function hook owns the address it is installed at, so where both fixes
//! correct through the same native callback there can only be one hook. Those
//! three sites are installed here and hand the widget to each fix in turn; the
//! corrections themselves stay in the module they belong to.

mod aspect_ratio;
mod background;
mod encounter;
mod fonts;
mod hud;
mod layout;
mod map;
mod minimap;
mod movies;
mod post_processing;
mod widgets;

use sdk::UMG::{ESlateVisibility, UWidget};

use crate::{
    config::Config,
    utils::{ModuleInfo, SignatureHook, inject_hook},
};

/// Applies every fix the config enables.
pub fn install(module: &ModuleInfo, config: &Config) {
    fonts::install(module, &config.fonts);
    aspect_ratio::apply(module);
    if config.background.enable {
        background::apply(module);
    }
    post_processing::install(module, config.post_processing);
    if config.hud.enable {
        // Re-framing the capture and holding the transition's layout to its
        // shape are halves of one correction, so both follow the HUD flag: a
        // re-framed capture drawn across an unconstrained layout would be
        // stretched by exactly what the re-frame trimmed.
        encounter::install(module);
        hud::install(module, config.hud.aspect());
    }
    // Poll the short menu-close grace on the GAME thread even while the
    // player is completely motionless. The existing UMG callbacks remain
    // the fallback if the frame probe cannot be installed.
    install_menu_frame_tick(module, config);
    hook_shared_widget_updates(module, config);
    if config.movies.enable {
        movies::install(module);
    }
}

/// Installs the native widget callbacks both fixes correct through, each
/// dispatching under the flag its own correction belongs to.
fn hook_shared_widget_updates(module: &ModuleInfo, config: &Config) {
    let (movies, hud) = (config.movies.enable, config.hud.enable);
    let aspect = config.hud.aspect();
    let minimap_probe = config.minimap.probe;
    let minimap_enabled = config.minimap.enable;
    let minimap_hint = config.minimap.name_hint.clone();
    let minimap_hint_rebuild = minimap_hint.clone();
    let minimap_hint_visibility = minimap_hint.clone();
    // Native UpdateRenderTransform. RCX is the widget, and the fields it reads
    // -- Translation at +0x90, Scale at +0xA0 -- are both consumed immediately
    // after entry, so a movie's scale and a tooltip's translation are written
    // here in place rather than through a setter that would recurse into it.
    // The frame size and stack offsets are wildcarded.
    if movies || hud || minimap_probe || minimap_enabled {
        let transform = SignatureHook {
            tag: "Widget render transform",
            signature: "48 8B C4 48 89 58 18 48 89 78 20 55 48 8D 68 ?? 48 81 EC ?? ?? ?? ?? 48 8D 55 ?? 48 89 70 10 48 8B D9 E8 ?? ?? ?? ??",
            offset: 0,
        };
        inject_hook(module, &transform, move |ctx| unsafe {
            let widget = ctx.rcx as *mut UWidget;
            if movies {
                movies::on_transform(widget);
            }
            if hud {
                hud::on_transform(widget, aspect);
                // The transition's capture image is the first widget to touch
                // the captured frame after the copy, which is where that frame
                // is re-framed to the shape the transition draws it at.
                encounter::reframe_capture(widget);
            }
            if minimap_probe || minimap_enabled {
                minimap::on_transform(widget, minimap_probe, minimap_enabled, &minimap_hint);
            }
        });
    }
    // Native UWidget::SetVisibility: RCX is the widget, DL the new visibility.
    // The read and write of Visibility at +0xDC stay literal; the frame size,
    // the registers they go through and the branch are wildcarded.
    let visibility = SignatureHook {
        tag: "Widget visibility change",
        signature: "48 89 5C 24 08 48 89 74 24 10 57 48 83 EC ?? 0F B6 ?? DC 00 00 00 48 8B ?? 0F B6 ?? ?? 3A ?? 74 ?? ?? 88 ?? DC 00 00 00",
        offset: 0,
    };
    let visibility_ready = (movies || hud)
        && inject_hook(module, &visibility, move |ctx| unsafe {
            let widget = ctx.rcx as *mut UWidget;
            let value = ESlateVisibility(ctx.rdx as u8);
            if minimap_enabled {
                // No per-widget trace: only monitor the two exact HUD classes.
                minimap::on_widget_visibility(widget, true);
                minimap::on_menu_visibility(widget, value, true, &minimap_hint_visibility);
            }
            if movies {
                movies::on_visibility(widget, value);
            }
            if hud {
                hud::expand_menu_backdrop(widget);
                map::update_backdrop(widget, Some(value));
            }
        });
    // Wrapping a movie root is only safe once the wrapper's dismissal is
    // synchronized, so that branch waits on the visibility hook; the dimming
    // scale carries no such dependency.
    let wrap_movies = movies && visibility_ready;
    if wrap_movies || hud {
        // Native UUserWidget rebuild, at the point RBX holds the widget and
        // before its tree root is consumed for the Slate rebuild. The
        // signature runs from the entry to the hook, so every byte before the
        // hook point is checked: MOV RBX,RCX, which puts the widget in RBX,
        // and the LEA RCX,[RBX+...] the hook lands on stay literal, while
        // frame sizes, stack slots, field offsets and the vtable slot are
        // wildcarded.
        let rebuild = SignatureHook {
            tag: "User widget rebuild",
            signature: "40 55 53 56 41 57 48 8D AC 24 ?? ?? ?? ?? 48 81 EC ?? ?? ?? ?? 48 8B 05 ?? ?? ?? ?? 48 33 C4 48 89 85 ?? ?? ?? ?? C7 44 24 ?? 00 00 00 00 4C 8B FA F6 81 ?? ?? ?? ?? ?? 48 8B D9 48 89 BC 24 ?? ?? ?? ?? 75 ?? 48 8B 01 FF 90 ?? ?? ?? ?? 48 8D 8B",
            offset: 0x4E,
        };
        inject_hook(module, &rebuild, move |ctx| unsafe {
            let widget = ctx.rbx as *mut UWidget;
            if minimap_probe || minimap_enabled {
                minimap::on_rebuild(widget, minimap_probe, minimap_enabled, &minimap_hint_rebuild);
            }
            if wrap_movies {
                movies::on_rebuild(widget.cast());
            }
            if hud {
                hud::expand_menu_backdrop(widget);
                hud::hide_battle_shadow(widget.cast());
                if visibility_ready {
                    map::update_backdrop(widget, None);
                }
            }
        });
    }
}


/// EXPERIMENTAL: FEngineLoop::Tick's idle-mode preamble executes once per
/// engine frame on the game thread (including while UI is open).
///
/// The signature is derived from PolarWizard's *existing* background idle
/// patch. That patch has already run before this hook is installed, so its
/// +6 conditional jump is EB in the enabled case, 74 in the disabled case.
/// We hook at +0 (XOR SIL,SIL; CMP [RAX],R13D), before the jump. The first
/// two complete instructions are six bytes, outside the +6 byte patch.
///
/// Requiring explicit opcodes rather than wildcarding these six bytes is
/// intentional: if an updated game has different instructions, we FAIL
/// CLOSED and fall back to the UMG callbacks rather than patch unknown code.
fn install_menu_frame_tick(module: &ModuleInfo, config: &Config) {
    if !config.minimap.enable
        || config.minimap.name_hint.trim() != "WBP_MiniMapLayout_C"
    {
        return;
    }
    let signature = if config.background.enable {
        "40 32 F6 44 39 28 EB ?? E8 ?? ?? ?? ?? 84 C0 75"
    } else {
        "40 32 F6 44 39 28 74 ?? E8 ?? ?? ?? ?? 84 C0 75"
    };
    let hook = SignatureHook {
        tag: "Menu restore game-thread frame tick (TEST ONLY)",
        signature,
        offset: 0,
    };
    let installed = inject_hook(module, &hook, move |_| {
        minimap::on_game_thread_tick();
    });
    if installed {
        log::info!("HUD_OPACITY_FRAME_HOOK installed=true (test candidate)");
    } else {
        log::warn!("HUD_OPACITY_FRAME_HOOK installed=false; retaining UMG fallback");
    }
}
