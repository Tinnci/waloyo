use crate::application::TaskService;
use crate::domain::{TaskId, TaskSubmitted};
use crate::presentation::animations::WaloyoAnimations;
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
    editing_task: Option<TaskId>,
    editing_buffer: SharedString,
    edit_focus_handle: FocusHandle,
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
        let edit_focus_handle = cx.focus_handle();

        Self {
            task_service: service,
            task_input,
            completing_task: None,
            clear_sky_celebration: false,
            editing_task: None,
            editing_buffer: "".into(),
            edit_focus_handle,
        }
    }

    fn add_task(&mut self, content: String, cx: &mut Context<Self>) {
        self.task_service.add_task(content);
        // Adding a task means we're no longer in clear sky
        self.clear_sky_celebration = false;
        cx.notify();
    }

    fn start_editing(&mut self, task_id: TaskId, content: SharedString, cx: &mut Context<Self>) {
        self.editing_task = Some(task_id);
        self.editing_buffer = content;
        cx.notify();
    }

    fn cancel_editing(&mut self, cx: &mut Context<Self>) {
        self.editing_task = None;
        self.editing_buffer = "".into();
        cx.notify();
    }

    fn save_editing(&mut self, cx: &mut Context<Self>) {
        if let Some(task_id) = self.editing_task {
            if !self.editing_buffer.is_empty() {
                self.task_service
                    .update_task_content(task_id, self.editing_buffer.clone());
            }
        }
        self.cancel_editing(cx);
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

    fn render_edit_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.edit_focus_handle.clone();

        div()
            .w_full()
            .px(px(Theme::PADDING_MD))
            .py(px(Theme::PADDING_SM))
            .bg(Theme::surface())
            .rounded(px(Theme::RADIUS_MD))
            .border_1()
            .border_color(Theme::accent_primary())
            .flex()
            .items_center()
            .gap(px(Theme::PADDING_SM))
            .child(
                div()
                    .w(px(12.0))
                    .h(px(12.0))
                    .rounded_full()
                    .bg(Theme::state_pending())
                    .opacity(0.5),
            )
            .child(
                div()
                    .flex_1()
                    .track_focus(&focus_handle)
                    .text_color(Theme::text_primary())
                    .child(self.editing_buffer.clone())
                    .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                        match &event.keystroke.key {
                            key if key == "enter" => {
                                this.save_editing(cx);
                            }
                            key if key == "escape" => {
                                this.cancel_editing(cx);
                            }
                            key if key == "backspace" => {
                                let mut s = this.editing_buffer.to_string();
                                s.pop();
                                this.editing_buffer = s.into();
                                cx.notify();
                            }
                            key if key == "space" => {
                                let mut s = this.editing_buffer.to_string();
                                s.push(' ');
                                this.editing_buffer = s.into();
                                cx.notify();
                            }
                            key if key.len() == 1 => {
                                let mut s = this.editing_buffer.to_string();
                                if event.keystroke.modifiers.shift {
                                    s.push_str(&key.to_uppercase());
                                } else {
                                    s.push_str(key);
                                }
                                this.editing_buffer = s.into();
                                cx.notify();
                            }
                            _ => {}
                        }
                    })), // Save on blur - wait, on_blur triggers when clicking ANYTHING else, including save button if we had one.
                         // But here clicking outside cancels? Or saves?
                         // Typically click outside saves.
                         // .on_blur(cx.listener(|this, _, _, cx| {
                         // this.save_editing(cx);
                         // }))
            )
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
                // ... (abbreviated, keeping original logic)
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
                    let entity_edit = entity.clone();

                    if Some(task.id) == self.editing_task {
                        self.render_edit_input(cx).into_any_element()
                    } else {
                        TaskItem::new(task.clone())
                            .on_complete(move |id, _window, cx| {
                                let _ = entity_complete.update(cx, |view, cx| {
                                    view.handle_task_click(id, cx);
                                });
                            })
                            .on_click_content(move |id, _window, cx| {
                                let _ = entity_edit.update(cx, |view, cx| {
                                    view.start_editing(id, task.content.clone(), cx);
                                });
                            })
                            .on_delete(move |id, _window, cx| {
                                let _ = entity_delete.update(cx, |view, cx| {
                                    view.delete_task(id, cx);
                                });
                            })
                            .into_any_element()
                    }
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
            .children(completed_tasks.into_iter().map(TaskItem::new))
            .into_any_element()
    }

    fn render_clear_sky_celebration(&self) -> impl IntoElement {
        div()
            .id("clear-sky-celebration")
            .absolute()
            .inset_0()
            .clear_sky("clear-sky-anim", self.clear_sky_celebration)
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
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                // Ctrl+Z for Undo
                if event.keystroke.modifiers.control && event.keystroke.key == "z" {
                    if this.task_service.undo() {
                        this.check_clear_sky(cx);
                        cx.notify();
                    }
                }
            }))
    }
}
