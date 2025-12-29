use crate::domain::TaskSubmitted;
use crate::presentation::theme::Theme;
use gpui::*;

/// A simple text input component for adding new tasks
pub struct TaskInput {
    focus_handle: FocusHandle,
    content: SharedString,
}

impl TaskInput {
    pub fn new(cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            content: "".into(),
        }
    }

    fn submit(&mut self, cx: &mut Context<Self>) {
        let content = self.content.to_string().trim().to_string();
        if !content.is_empty() {
            cx.emit(TaskSubmitted(content));
            self.content = "".into();
        }
    }
}

impl EventEmitter<TaskSubmitted> for TaskInput {}

impl Render for TaskInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();

        div()
            .id("task-input-container")
            .w_full()
            .px(px(Theme::PADDING_LG))
            .py(px(Theme::PADDING_SM))
            .child(
                div()
                    .id("task-input")
                    .track_focus(&focus_handle)
                    .w_full()
                    .px(px(Theme::PADDING_MD))
                    .py(px(Theme::PADDING_SM))
                    .bg(Theme::surface())
                    .rounded(px(Theme::RADIUS_MD))
                    .border_1()
                    .border_color(rgba(0xffffff10))
                    .flex()
                    .items_center()
                    .gap(px(Theme::PADDING_SM))
                    .focus(|style| style.border_color(Theme::accent_primary()))
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
                            .text_color(if self.content.is_empty() {
                                Theme::text_secondary()
                            } else {
                                Theme::text_primary()
                            })
                            .child(if self.content.is_empty() {
                                "Add a new task to overcome...".into()
                            } else {
                                self.content.clone()
                            }),
                    )
                    .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                        match &event.keystroke.key {
                            key if key == "enter" => {
                                this.submit(cx);
                                cx.notify();
                            }
                            key if key == "backspace" => {
                                let mut content = this.content.to_string();
                                content.pop();
                                this.content = content.into();
                                cx.notify();
                            }
                            key if key == "space" => {
                                let mut content = this.content.to_string();
                                content.push(' ');
                                this.content = content.into();
                                cx.notify();
                            }
                            key if key.len() == 1 => {
                                let mut content = this.content.to_string();
                                if event.keystroke.modifiers.shift {
                                    content.push_str(&key.to_uppercase());
                                } else {
                                    content.push_str(key);
                                }
                                this.content = content.into();
                                cx.notify();
                            }
                            _ => {}
                        }
                    })),
            )
    }
}
