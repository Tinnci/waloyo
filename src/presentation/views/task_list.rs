use crate::application::TaskService;
use crate::domain::TaskId;
use crate::presentation::components::TaskItem;
use crate::presentation::theme::Theme;
use gpui::*;

/// The main task list view
/// Displays pending tasks at the top and completed tasks at the bottom
pub struct TaskListView {
    task_service: TaskService,
}

impl TaskListView {
    pub fn new() -> Self {
        let mut service = TaskService::new();

        // Add some demo tasks
        service.add_task("Learn GPUI fundamentals");
        service.add_task("Build Waloyo task manager");
        service.add_task("Implement rain drop animation");
        service.add_task("Add wind swaying effect");
        service.add_task("Create clear sky celebration");

        Self {
            task_service: service,
        }
    }

    fn handle_task_click(&mut self, task_id: TaskId, cx: &mut Context<Self>) {
        // Start the completing animation
        if self.task_service.begin_completing(task_id) {
            cx.notify();

            // After animation, mark as done
            // In real implementation, this would be triggered by animation end
            self.task_service.finish_completing(task_id);
            cx.notify();
        }
    }

    fn render_header(&self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
            .child(
                div()
                    .text_sm()
                    .text_color(Theme::text_secondary())
                    .child(format!("{} pending · {} overcome", pending, completed)),
            )
    }

    fn render_task_list(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tasks: Vec<_> = self.task_service.all_tasks().to_vec();
        let entity = cx.entity().downgrade();

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
            .children(tasks.into_iter().map({
                let entity = entity.clone();
                move |task| {
                    let task_id = task.id;
                    let entity = entity.clone();
                    TaskItem::new(task).on_complete(move |id, _window, cx| {
                        let _ = entity.update(cx, |view, cx| {
                            view.handle_task_click(id, cx);
                        });
                    })
                }
            }))
    }
}

impl Render for TaskListView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let all_done = self.task_service.all_overcome();

        let bg_color = if all_done {
            Theme::clear_sky_background()
        } else {
            Theme::background()
        };

        div()
            .size_full()
            .bg(bg_color)
            .flex()
            .flex_col()
            .child(self.render_header(window, cx))
            .child(self.render_task_list(window, cx))
    }
}
