use crate::application::TaskService;
use crate::domain::{TaskId, TaskSubmitted};
use crate::presentation::components::{TaskInput, TaskItem};
use crate::presentation::theme::Theme;
use gpui::*;
use std::time::Duration;

/// The main task list view
/// Displays pending tasks at the top and completed tasks at the bottom
pub struct TaskListView {
    task_service: TaskService,
    task_input: Entity<TaskInput>,
    #[allow(dead_code)]
    completing_task: Option<TaskId>,
    clear_sky_celebration: bool,
}

impl TaskListView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // Create the task input
        let task_input = cx.new(|cx| TaskInput::new(cx));

        // Subscribe to task submissions
        cx.subscribe(&task_input, |this, _input, event: &TaskSubmitted, cx| {
            this.add_task(event.0.clone(), cx);
        })
        .detach();

        // Load tasks from storage (or create demo tasks if empty)
        let service = TaskService::default();

        Self {
            task_service: service,
            task_input,
            completing_task: None,
            clear_sky_celebration: false,
        }
    }

    fn add_task(&mut self, content: String, cx: &mut Context<Self>) {
        self.task_service.add_task(content);
        // Adding a task means we're no longer in clear sky
        self.clear_sky_celebration = false;
        cx.notify();
    }

    fn delete_task(&mut self, task_id: TaskId, cx: &mut Context<Self>) {
        self.task_service.remove_task(task_id);
        self.check_clear_sky(cx);
        cx.notify();
    }

    fn handle_task_click(&mut self, task_id: TaskId, cx: &mut Context<Self>) {
        // Start the completing animation
        if self.task_service.begin_completing(task_id) {
            self.completing_task = Some(task_id);
            cx.notify();

            // Schedule completion after animation
            let entity = cx.entity().downgrade();
            cx.spawn(async move |_weak_entity, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(Theme::ANIM_RAIN_DROP))
                    .await;

                let _ = entity.update(cx, |view, cx| {
                    view.task_service.finish_completing(task_id);
                    view.completing_task = None;
                    view.check_clear_sky(cx);
                    cx.notify();
                });
            })
            .detach();
        }
    }

    fn check_clear_sky(&mut self, cx: &mut Context<Self>) {
        if self.task_service.all_overcome() && !self.clear_sky_celebration {
            self.clear_sky_celebration = true;
            cx.notify();
        }
    }

    fn render_header(&self) -> impl IntoElement {
        let pending = self.task_service.pending_count();
        let completed = self.task_service.completed_count();
        let all_done = self.task_service.all_overcome();

        div()
            .w_full()
            .px(px(Theme::PADDING_LG))
            .py(px(Theme::PADDING_MD))
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(if all_done {
                                Theme::clear_sky_accent()
                            } else {
                                Theme::text_primary()
                            })
                            .child("Waloyo"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::text_accent())
                            .child("We Overcome"),
                    ),
            )
            .child(div().text_sm().text_color(Theme::text_secondary()).child(
                if all_done && completed > 0 {
                    format!("🎉 All {} tasks overcome! Clear skies ahead!", completed)
                } else {
                    format!("{} pending · {} overcome", pending, completed)
                },
            ))
    }

    fn render_task_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let tasks: Vec<_> = self.task_service.all_tasks().to_vec();
        let entity = cx.entity().downgrade();

        let pending_tasks: Vec<_> = tasks.iter().filter(|t| !t.is_done()).cloned().collect();

        if pending_tasks.is_empty() {
            return div()
                .id("task-list-container")
                .w_full()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_color(Theme::text_secondary())
                        .text_center()
                        .child(if self.task_service.completed_count() > 0 {
                            "☀️ Clear skies! All tasks overcome."
                        } else {
                            "No tasks yet. Add one above!"
                        }),
                )
                .into_any_element();
        }

        div()
            .id("task-list-container")
            .w_full()
            .flex_1()
            .overflow_y_scroll()
            .px(px(Theme::PADDING_LG))
            .py(px(Theme::PADDING_SM))
            .flex()
            .flex_col()
            .gap_2()
            .children(pending_tasks.into_iter().map({
                let entity = entity.clone();
                move |task| {
                    let entity_complete = entity.clone();
                    let entity_delete = entity.clone();
                    let task_id = task.id;

                    TaskItem::new(task)
                        .on_complete(move |id, _window, cx| {
                            let _ = entity_complete.update(cx, |view, cx| {
                                view.handle_task_click(id, cx);
                            });
                        })
                        .on_delete(move |id, _window, cx| {
                            let _ = entity_delete.update(cx, |view, cx| {
                                view.delete_task(id, cx);
                            });
                        })
                }
            }))
            .into_any_element()
    }

    fn render_completed_section(&self) -> impl IntoElement {
        let completed_tasks: Vec<_> = self
            .task_service
            .all_tasks()
            .iter()
            .filter(|t| t.is_done())
            .cloned()
            .collect();

        if completed_tasks.is_empty() {
            return div().into_any_element();
        }

        div()
            .w_full()
            .px(px(Theme::PADDING_LG))
            .py(px(Theme::PADDING_SM))
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_xs()
                    .text_color(Theme::text_secondary())
                    .mb_2()
                    .child(format!("✓ Overcome ({})", completed_tasks.len())),
            )
            .children(completed_tasks.into_iter().map(|task| TaskItem::new(task)))
            .into_any_element()
    }

    fn render_clear_sky_celebration(&self) -> impl IntoElement {
        if !self.clear_sky_celebration {
            return div().into_any_element();
        }

        // Simple celebration overlay that fades in
        div()
            .id("clear-sky-celebration")
            .absolute()
            .inset_0()
            .with_animation(
                "clear-sky-anim",
                Animation::new(Duration::from_millis(2000)).with_easing(ease_in_out),
                |element, delta| {
                    // Fade in a subtle golden glow effect
                    let opacity = delta * 0.08;
                    element.bg(rgba(0xffc77700 + ((opacity * 255.0) as u32)))
                },
            )
            .into_any_element()
    }
}

impl Render for TaskListView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let all_done = self.task_service.all_overcome();

        // Background color with Clear Sky mode
        let bg = if all_done && self.clear_sky_celebration {
            Theme::clear_sky_background()
        } else {
            Theme::background()
        };

        div()
            .size_full()
            .bg(bg)
            .relative()
            .flex()
            .flex_col()
            .child(self.render_clear_sky_celebration())
            .child(self.render_header())
            .child(self.task_input.clone())
            .child(self.render_task_list(cx))
            .child(self.render_completed_section())
    }
}
