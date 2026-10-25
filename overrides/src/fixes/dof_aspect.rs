//! Auto DOF aspect source shared by the original HUD viewport hook and the
//! final post-process hook. No additional game/native hooks are installed.
//!
//! Actual UMG viewport size is authoritative once available. The saved
//! GameUserSettings resolution seeds DOF before UI initialization and when
//! the HUD fix is disabled. It is NOT a guarantee for unusual window modes.

use std::{
    fs,
    sync::atomic::{AtomicU64, Ordering},
};

const BASE_ASPECT: f64 = 16.0 / 9.0;
// EXACT two-anchor power-law fit:
// scale(16:9)=1 and scale(3440:1440)=1.5:
// exponent = ln(1.5) / ln((3440/1440)/(16/9)) = ln(1.5)/ln(43/32).
// This is a mathematical interpolation/extrapolation of ONE empirically
// calibrated ultrawide sample, NOT proof of exact optics at other aspects.
const SENSOR_EXPONENT: f64 = 1.372_298_540_445_753_5;
const MAX_UNVALIDATED_SCALE: f64 = 4.0;

// High 32 bits are viewport width, low 32 bits its height.
// A single atomic avoids racing a width update against a height update.
static VIEWPORT: AtomicU64 = AtomicU64::new(0);

fn store_size(width: u32, height: u32) {
    if (320..=32768).contains(&width) && (200..=16384).contains(&height) {
        VIEWPORT.store((u64::from(width) << 32) | u64::from(height), Ordering::Relaxed);
    }
}

/// Reads the actual on-screen game UI viewport once the main HUD root is
/// attached. DPI scale cancels when width / height is formed.
pub(super) fn record_viewport(width: f64, height: f64) {
    if width.is_finite()
        && height.is_finite()
        && width >= 320.0
        && height >= 200.0
        && width <= 32768.0
        && height <= 16384.0
    {
        store_size(width.round() as u32, height.round() as u32);
    }
}

/// Startup fallback for loading views before the HUD exists. Reads only the
/// user's existing game resolution settings; never creates/modifies files.
pub(super) fn seed_game_settings() {
    let Ok(local) = std::env::var("LOCALAPPDATA") else {
        log::warn!("DOF_AUTO: no LOCALAPPDATA; waiting for HUD viewport");
        return;
    };
    let file = std::path::Path::new(&local)
        .join("FFRS")
        .join("Saved")
        .join("Config")
        .join("Windows")
        .join("GameUserSettings.ini");
    let Ok(body) = fs::read_to_string(&file) else {
        log::info!("DOF_AUTO: no saved resolution; waiting for HUD viewport");
        return;
    };
    let mut section = false;
    let (mut width, mut height) = (None, None);
    for line in body.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            section = line.eq_ignore_ascii_case("[/Script/Engine.GameUserSettings]");
        } else if section {
            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "ResolutionSizeX" => width = value.trim().parse::<u32>().ok(),
                    "ResolutionSizeY" => height = value.trim().parse::<u32>().ok(),
                    _ => {}
                }
            }
        }
    }
    if let (Some(w), Some(h)) = (width, height) {
        store_size(w, h);
        log::info!("DOF_AUTO: startup saved-resolution fallback {}x{}", w, h);
    }
}

/// Returns (measured/fallback aspect, bounded optical compensation).
/// Always fails closed to unmodified sensor width on invalid/unavailable
/// measurements, 16:9 or narrower screens, and numerical overflow.
pub(super) fn correction() -> (f64, f32) {
    let packed = VIEWPORT.load(Ordering::Relaxed);
    let (w, h) = ((packed >> 32) as u32, packed as u32);
    if w == 0 || h == 0 {
        return (BASE_ASPECT, 1.0);
    }
    let aspect = f64::from(w) / f64::from(h);
    if !aspect.is_finite() || aspect <= BASE_ASPECT + 0.001 {
        return (aspect, 1.0);
    }
    let relative = aspect / BASE_ASPECT;
    let optical = relative.powf(SENSOR_EXPONENT);
    if !optical.is_finite() {
        return (aspect, 1.0);
    }
    (aspect, optical.clamp(1.0, MAX_UNVALIDATED_SCALE) as f32)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calibrated_anchors_and_fallback() {
        VIEWPORT.store(0, Ordering::Relaxed);
        assert_eq!(correction().1, 1.0);
        record_viewport(1920.0, 1080.0);
        assert_eq!(correction().1, 1.0);
        record_viewport(3440.0, 1440.0);
        assert!((correction().1 - 1.5).abs() < 0.00001);
        record_viewport(5120.0, 1440.0);
        assert!((f64::from(correction().1) - 2.5888269659).abs() < 0.00001);
        record_viewport(7680.0, 2160.0);
        assert!((f64::from(correction().1) - 2.5888269659).abs() < 0.00001);
        record_viewport(10240.0, 1440.0);
        assert_eq!(correction().1, 4.0);
    }
}
