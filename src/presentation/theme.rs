use gpui::{rgb, rgba, Rgba};

/// Waloyo Theme - "Wind & Rain" color palette
///
/// The theme follows these metaphors:
/// - Storm colors: Dark, moody backgrounds representing challenges
/// - Wind colors: Subtle grays and blues for pending tasks  
/// - Rain colors: Deep blues for completing animations
/// - Clear sky: Bright, peaceful colors when all tasks are done
pub struct Theme;

impl Theme {
    // ═══════════════════════════════════════════════════════════════════
    // Background Colors - The Storm
    // ═══════════════════════════════════════════════════════════════════

    /// Main background - deep stormy night
    pub fn background() -> Rgba {
        rgb(0x1a1b26)
    }

    /// Surface background - slightly lighter for cards
    pub fn surface() -> Rgba {
        rgb(0x24283b)
    }

    /// Elevated surface - for hover states
    pub fn surface_hover() -> Rgba {
        rgb(0x2f3549)
    }

    // ═══════════════════════════════════════════════════════════════════
    // Text Colors
    // ═══════════════════════════════════════════════════════════════════

    /// Primary text - soft white
    pub fn text_primary() -> Rgba {
        rgb(0xa9b1d6)
    }

    /// Secondary text - muted
    pub fn text_secondary() -> Rgba {
        rgb(0x565f89)
    }

    /// Accent text - rain blue
    pub fn text_accent() -> Rgba {
        rgb(0x7aa2f7)
    }

    // ═══════════════════════════════════════════════════════════════════
    // State Colors - Wind & Rain
    // ═══════════════════════════════════════════════════════════════════

    /// Pending state - wind gray with blue tint
    pub fn state_pending() -> Rgba {
        rgb(0x414868)
    }

    /// Completing state - rain blue (animated)
    pub fn state_completing() -> Rgba {
        rgb(0x7aa2f7)
    }

    /// Done state - clear sky green
    pub fn state_done() -> Rgba {
        rgb(0x9ece6a)
    }

    /// High priority - storm red
    pub fn priority_high() -> Rgba {
        rgb(0xf7768e)
    }

    /// High priority background - 10% opacity storm red
    pub fn priority_high_bg() -> Rgba {
        rgba(0xf7768e1a)
    }

    /// Medium priority - lightning yellow
    pub fn priority_medium() -> Rgba {
        rgb(0xe0af68)
    }

    /// Medium priority background - 10% opacity lightning yellow
    pub fn priority_medium_bg() -> Rgba {
        rgba(0xe0af681a)
    }

    /// Low priority - gentle breeze
    pub fn priority_low() -> Rgba {
        rgb(0x565f89)
    }

    /// Low priority background - 10% opacity gentle breeze
    pub fn priority_low_bg() -> Rgba {
        rgba(0x565f891a)
    }

    // ═══════════════════════════════════════════════════════════════════
    // Accent Colors
    // ═══════════════════════════════════════════════════════════════════

    /// Primary accent - electric blue
    pub fn accent_primary() -> Rgba {
        rgb(0x7aa2f7)
    }

    /// Warning - lightning yellow
    pub fn accent_warning() -> Rgba {
        rgb(0xe0af68)
    }

    /// Error - storm red
    pub fn accent_error() -> Rgba {
        rgb(0xf7768e)
    }

    // ═══════════════════════════════════════════════════════════════════
    // Clear Sky Mode - When all tasks are done
    // ═══════════════════════════════════════════════════════════════════

    /// Clear sky background - peaceful dawn
    pub fn clear_sky_background() -> Rgba {
        rgb(0x1a1f36)
    }

    /// Clear sky accent - sunrise gold
    pub fn clear_sky_accent() -> Rgba {
        rgb(0xffc777)
    }

    // ═══════════════════════════════════════════════════════════════════
    // Spacing & Sizing
    // ═══════════════════════════════════════════════════════════════════

    /// Standard padding for components
    pub const PADDING_SM: f32 = 8.0;
    pub const PADDING_MD: f32 = 16.0;
    pub const PADDING_LG: f32 = 24.0;

    /// Border radius
    pub const RADIUS_SM: f32 = 4.0;
    pub const RADIUS_MD: f32 = 8.0;
    pub const RADIUS_LG: f32 = 12.0;

    /// Animation durations (ms)
    pub const ANIM_FAST: u64 = 150;
    pub const ANIM_NORMAL: u64 = 300;
    pub const ANIM_SLOW: u64 = 600;
    pub const ANIM_RAIN_DROP: u64 = 800;
}
