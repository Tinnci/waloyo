use crate::domain::{Task, TaskId};
use crate::presentation::animations::WaloyoAnimations;
use crate::presentation::theme::Theme;
use gpui::prelude::*;
use gpui::*;

/// Type alias for task event handlers
pub type TaskEventHandler = Box<dyn Fn(TaskId, &mut Window, &mut App) + 'static>;

/// A single task item component - the "wind" element
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

        // Prepare handlers
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
            if let Some(handler) = on_complete {
                indicator = indicator.cursor_pointer().on_mouse_down(
                    MouseButton::Left,
                    move |_event, window, cx| {
                        handler(task_id, window, cx);
                    },
                );
            }
        }

        // Build Content Area
        let mut content_area = div().flex_1().flex().flex_col().gap_1().child(
            div()
                .text_color(content_color)
                .when(is_done, |this| this.line_through())
                .child(self.task.content.clone()),
        );

        // Metadata row (Priority & Due Date)
        if !is_done && !is_completing {
            let (priority_color, priority_bg) = match self.task.priority {
                crate::domain::TaskPriority::High => {
                    (Theme::priority_high(), Theme::priority_high_bg())
                }
                crate::domain::TaskPriority::Medium => {
                    (Theme::priority_medium(), Theme::priority_medium_bg())
                }
                crate::domain::TaskPriority::Low => {
                    (Theme::priority_low(), Theme::priority_low_bg())
                }
            };

            let mut meta_row = div().flex().items_center().gap_3();

            // Priority dot
            meta_row = meta_row.child(
                div()
                    .px_1()
                    .py_0()
                    .rounded(px(Theme::RADIUS_SM))
                    .bg(priority_bg)
                    .text_color(priority_color)
                    .text_xs()
                    .child(match self.task.priority {
                        crate::domain::TaskPriority::High => "High",
                        crate::domain::TaskPriority::Medium => "Medium",
                        crate::domain::TaskPriority::Low => "Low",
                    }),
            );

            // Due Date
            if let Some(due_date) = self.task.due_date {
                let now = chrono::Local::now();
                let today = now.date_naive();
                let due_naive = due_date.date_naive();

                let date_str = if due_naive == today {
                    "Today".to_string()
                } else if due_naive == today.succ_opt().unwrap_or(today) {
                    "Tomorrow".to_string()
                } else if due_naive < today {
                    format!("Overdue: {}", due_naive.format("%m/%d"))
                } else {
                    due_naive.format("%m/%d").to_string()
                };

                let is_overdue = due_naive < today;

                meta_row = meta_row.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .text_xs()
                        .text_color(if is_overdue {
                            Theme::accent_error()
                        } else {
                            Theme::text_secondary()
                        })
                        .child("📅")
                        .child(date_str),
                );
            }

            // Notes indicator
            if self.task.notes.is_some() {
                meta_row = meta_row.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .text_xs()
                        .text_color(Theme::text_secondary())
                        .child("📝"),
                );
            }

            content_area = content_area.child(meta_row);
        }

        if is_pending {
            if let Some(handler) = on_click_content {
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
            on_delete.map(|handler| {
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
                    })
            })
        } else {
            None
        };

        // Build the base card
        let base = div()
            .id(ElementId::Name(format!("task-item-{}", task_id.0).into()))
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
            .child(content_area)
            .when_some(delete_btn, |this, btn| this.child(btn));

        // Apply Metaphorical Animations (Mutually Exclusive)
        if is_pending {
            base.wind_sway(ElementId::Name(format!("sway-{}", task_id.0).into()), true)
        } else if is_completing {
            base.rain_drop(ElementId::Name(format!("rain-{}", task_id.0).into()), true)
        } else {
            base.into_any_element()
        }
    }
}
