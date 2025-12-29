use crate::domain::{Task, TaskId, TaskState};
use crate::presentation::theme::Theme;
use gpui::prelude::*; // Import prelude for InteractiveElement (on_click)
use gpui::*;
use std::f32::consts::PI;
use std::time::Duration;

/// Type alias for task event handlers
pub type TaskEventHandler = Box<dyn Fn(TaskId, &mut Window, &mut App) + 'static>;

/// A single task item component - the "wind" element
///
/// Visual metaphor:
/// - Pending: A card that sways gently like something carried by wind
/// - Completing: The card transforms into a rain drop and falls
/// - Done: Faded, peaceful appearance
#[derive(IntoElement)]
pub struct TaskItem {
    task: Task,
    on_complete: Option<TaskEventHandler>,
    on_delete: Option<TaskEventHandler>,
    on_click_content: Option<TaskEventHandler>,
}

impl TaskItem {
    pub fn new(task: Task) -> Self {
        Self {
            task,
            on_complete: None,
            on_delete: None,
            on_click_content: None,
        }
    }

    pub fn on_complete(
        mut self,
        handler: impl Fn(TaskId, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_complete = Some(Box::new(handler));
        self
    }

    pub fn on_delete(mut self, handler: impl Fn(TaskId, &mut Window, &mut App) + 'static) -> Self {
        self.on_delete = Some(Box::new(handler));
        self
    }

    pub fn on_click_content(
        mut self,
        handler: impl Fn(TaskId, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click_content = Some(Box::new(handler));
        self
    }

    fn state_indicator(&self) -> Div {
        let (bg_color, size) = match self.task.state {
            TaskState::Pending => (Theme::state_pending(), 12.0),
            TaskState::Completing => (Theme::state_completing(), 14.0),
            TaskState::Done => (Theme::state_done(), 12.0),
        };

        div()
            .w(px(size))
            .h(px(size))
            .rounded_full()
            .bg(bg_color)
            .flex_shrink_0()
    }

    fn render_delete_button(&self, task_id: TaskId) -> Option<impl IntoElement> {
        self.on_delete.as_ref().map(|_| {
            div()
                .id(ElementId::Name(format!("delete-{}", task_id.0).into()))
                .w(px(24.0))
                .h(px(24.0))
                .rounded(px(4.0))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .text_color(Theme::text_secondary())
                .hover(|s| s.bg(rgba(0xff000020)).text_color(Theme::accent_error()))
                .child("×")
        })
    }
}

/// Wind sway easing function - oscillates smoothly between 0 and 1
fn wind_sway_easing(delta: f32) -> f32 {
    // Use sine wave to create smooth oscillation
    // Maps 0..1 to 0..2π for one complete cycle
    // Then maps -1..1 back to 0..1 range for the animation system
    let oscillation = (delta * 2.0 * PI).sin();
    (oscillation + 1.0) / 2.0 // Map from -1..1 to 0..1
}

impl RenderOnce for TaskItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let task_id = self.task.id;
        let is_pending = self.task.is_pending();
        let is_completing = self.task.is_completing();
        let is_done = self.task.is_done();

        let content_color = if is_done {
            Theme::text_secondary()
        } else {
            Theme::text_primary()
        };

        let card_bg = if is_completing {
            Theme::state_completing()
        } else {
            Theme::surface()
        };

        // Prepare handlers - wrap in Arc to allow sharing in closures
        let on_complete = self.on_complete.map(std::sync::Arc::new);
        let on_delete = self.on_delete.map(std::sync::Arc::new);
        let on_click_content = self.on_click_content.map(std::sync::Arc::new);

        // Build state indicator
        let mut indicator = div()
            .w(px(if is_completing { 14.0 } else { 12.0 }))
            .h(px(if is_completing { 14.0 } else { 12.0 }))
            .rounded_full()
            .bg(if is_completing {
                Theme::state_completing()
            } else if is_done {
                Theme::state_done()
            } else {
                Theme::state_pending()
            })
            .flex_shrink_0();

        if is_pending {
            if let Some(handler) = on_complete.clone() {
                indicator = indicator.cursor_pointer().on_mouse_down(
                    MouseButton::Left,
                    move |_event, window, cx| {
                        handler(task_id, window, cx);
                    },
                );
            }
        }

        // Build Content Area
        let mut content_area = div()
            .flex_1()
            .text_color(content_color)
            .when(is_done, |this| this.line_through())
            .child(self.task.content.clone());

        if is_pending {
            if let Some(handler) = on_click_content.clone() {
                content_area = content_area.cursor_pointer().on_mouse_down(
                    MouseButton::Left,
                    move |_event, window, cx| {
                        handler(task_id, window, cx);
                    },
                );
            }
        }

        // Build Delete Button
        let delete_btn = if !is_completing {
            if let Some(handler) = on_delete.clone() {
                Some(
                    div()
                        .id(ElementId::Name(format!("delete-{}", task_id.0).into()))
                        .w(px(24.0))
                        .h(px(24.0))
                        .rounded(px(4.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .text_color(Theme::text_secondary())
                        .hover(|s| s.bg(rgba(0xff000020)).text_color(Theme::accent_error()))
                        .child("×")
                        .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                            handler(task_id, window, cx);
                        }),
                )
            } else {
                None
            }
        } else {
            None
        };

        // Build the base card
        let mut base = div()
            .w_full()
            .px(px(Theme::PADDING_MD))
            .py(px(Theme::PADDING_SM))
            .bg(card_bg)
            .rounded(px(Theme::RADIUS_MD))
            .border_1()
            .border_color(rgba(0xffffff10))
            .hover(|style| style.bg(Theme::surface_hover()))
            .flex()
            .items_center()
            .gap(px(Theme::PADDING_SM))
            .child(indicator)
            .child(content_area);

        if let Some(btn) = delete_btn {
            base = base.child(btn);
        }

        let base_with_id = base.id(ElementId::Name(format!("task-{}", task_id.0).into()));

        // Apply different animations based on state
        if is_completing {
            // Rain Drop animation - fall down and fade
            base_with_id
                .with_animation(
                    ElementId::Name(format!("rain-drop-{}", task_id.0).into()),
                    Animation::new(Duration::from_millis(Theme::ANIM_RAIN_DROP))
                        .with_easing(ease_in_out),
                    move |element, delta| {
                        let fall_distance = 80.0 * delta;
                        let opacity_val = 1.0 - (delta * 0.7);
                        element.mt(px(fall_distance)).opacity(opacity_val)
                    },
                )
                .into_any_element()
        } else if is_pending {
            // Wind Sway animation
            base_with_id
                .with_animation(
                    ElementId::Name(format!("wind-sway-{}", task_id.0).into()),
                    Animation::new(Duration::from_millis(3000))
                        .repeat()
                        .with_easing(wind_sway_easing),
                    move |element, delta| {
                        let sway_offset = (delta - 0.5) * 6.0;
                        element.ml(px(sway_offset))
                    },
                )
                .into_any_element()
        } else {
            // Done state
            base_with_id.into_any_element()
        }
    }
}
