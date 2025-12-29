use crate::domain::{Task, TaskId, TaskState};
use crate::presentation::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::*;
use std::time::Duration;

/// A single task item component - the "wind" element
///
/// Visual metaphor:
/// - Pending: A card that sways gently like something carried by wind
/// - Completing: The card transforms into a rain drop and falls
/// - Done: Faded, peaceful appearance
#[derive(IntoElement)]
pub struct TaskItem {
    task: Task,
    on_complete: Option<Box<dyn Fn(TaskId, &mut Window, &mut App) + 'static>>,
}

impl TaskItem {
    pub fn new(task: Task) -> Self {
        Self {
            task,
            on_complete: None,
        }
    }

    pub fn on_complete(
        mut self,
        handler: impl Fn(TaskId, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_complete = Some(Box::new(handler));
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
}

impl RenderOnce for TaskItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let task_id = self.task.id;
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

        let base = div()
            .w_full()
            .px(px(Theme::PADDING_MD))
            .py(px(Theme::PADDING_SM))
            .bg(card_bg)
            .rounded(px(Theme::RADIUS_MD))
            .border_1()
            .border_color(rgba(0xffffff10))
            .cursor_pointer()
            .hover(|style| style.bg(Theme::surface_hover()))
            .flex()
            .items_center()
            .gap(px(Theme::PADDING_SM))
            .child(self.state_indicator())
            .child(
                div()
                    .flex_1()
                    .text_color(content_color)
                    .when(is_done, |this| this.line_through())
                    .child(self.task.content.clone()),
            );

        let base_with_id = base.id(ElementId::Name(format!("task-static-{}", task_id.0).into()));

        // Add rain drop animation when completing
        if is_completing {
            base_with_id
                .with_animation(
                    ElementId::Name(format!("rain-drop-{}", task_id.0).into()),
                    Animation::new(Duration::from_millis(Theme::ANIM_RAIN_DROP))
                        .with_easing(ease_in_out),
                    move |element, delta| {
                        // Rain drop effect: fall down and fade out
                        let fall_distance = 80.0 * delta;
                        let opacity_val = 1.0 - (delta * 0.7);

                        element.mt(px(fall_distance)).opacity(opacity_val)
                    },
                )
                .into_any_element()
        } else if let Some(handler) = self.on_complete {
            base_with_id
                .on_click(move |_event, window, cx| {
                    handler(task_id, window, cx);
                })
                .into_any_element()
        } else {
            base_with_id.into_any_element()
        }
    }
}
