use crate::presentation::theme::Theme;
use gpui::*;
use std::f32::consts::PI;
use std::time::Duration;

/// Easing function for the "Wind Sway" effect.
/// Oscillates smoothly between 0 and 1.
pub fn wind_sway_easing(delta: f32) -> f32 {
    let oscillation = (delta * 2.0 * PI).sin();
    (oscillation + 1.0) / 2.0
}

/// A trait to add metaphorical animations to elements.
pub trait WaloyoAnimations: Sized {
    /// Apply the "Wind Sway" animation - a gentle horizontal oscillation.
    fn wind_sway(self, id: impl Into<ElementId>, active: bool) -> AnyElement;

    /// Apply the "Rain Drop" animation - falling and fading.
    fn rain_drop(self, id: impl Into<ElementId>, active: bool) -> AnyElement;

    /// Apply the "Clear Sky" celebration effect - a subtle golden glow.
    fn clear_sky(self, id: impl Into<ElementId>, active: bool) -> AnyElement;
}

impl WaloyoAnimations for Div {
    fn wind_sway(self, id: impl Into<ElementId>, active: bool) -> AnyElement {
        if !active {
            return self.into_any_element();
        }

        self.with_animation(
            id,
            Animation::new(Duration::from_millis(3000))
                .repeat()
                .with_easing(wind_sway_easing),
            move |element, delta| {
                let sway_offset = (delta - 0.5) * 6.0;
                element.ml(px(sway_offset))
            },
        )
        .into_any_element()
    }

    fn rain_drop(self, id: impl Into<ElementId>, active: bool) -> AnyElement {
        if !active {
            return self.into_any_element();
        }

        self.with_animation(
            id,
            Animation::new(Duration::from_millis(Theme::ANIM_RAIN_DROP)).with_easing(ease_in_out),
            move |element, delta| {
                let fall_distance = 80.0 * delta;
                let opacity_val = 1.0 - (delta * 0.7);
                element.mt(px(fall_distance)).opacity(opacity_val)
            },
        )
        .into_any_element()
    }

    fn clear_sky(self, id: impl Into<ElementId>, active: bool) -> AnyElement {
        if !active {
            return self.into_any_element();
        }

        self.with_animation(
            id,
            Animation::new(Duration::from_millis(2000)).with_easing(ease_in_out),
            |element, delta| {
                let opacity = delta * 0.08;
                element.bg(rgba(0xffc77700 + ((opacity * 255.0) as u32)))
            },
        )
        .into_any_element()
    }
}

impl WaloyoAnimations for Stateful<Div> {
    fn wind_sway(self, id: impl Into<ElementId>, active: bool) -> AnyElement {
        if !active {
            return self.into_any_element();
        }

        self.with_animation(
            id,
            Animation::new(Duration::from_millis(3000))
                .repeat()
                .with_easing(wind_sway_easing),
            move |element, delta| {
                let sway_offset = (delta - 0.5) * 6.0;
                element.ml(px(sway_offset))
            },
        )
        .into_any_element()
    }

    fn rain_drop(self, id: impl Into<ElementId>, active: bool) -> AnyElement {
        if !active {
            return self.into_any_element();
        }

        self.with_animation(
            id,
            Animation::new(Duration::from_millis(Theme::ANIM_RAIN_DROP)).with_easing(ease_in_out),
            move |element, delta| {
                let fall_distance = 80.0 * delta;
                let opacity_val = 1.0 - (delta * 0.7);
                element.mt(px(fall_distance)).opacity(opacity_val)
            },
        )
        .into_any_element()
    }

    fn clear_sky(self, id: impl Into<ElementId>, active: bool) -> AnyElement {
        if !active {
            return self.into_any_element();
        }

        self.with_animation(
            id,
            Animation::new(Duration::from_millis(2000)).with_easing(ease_in_out),
            |element, delta| {
                let opacity = delta * 0.08;
                element.bg(rgba(0xffc77700 + ((opacity * 255.0) as u32)))
            },
        )
        .into_any_element()
    }
}
