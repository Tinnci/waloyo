// Waloyo - We Overcome
// A task management app built with GPUI following DDD architecture
//
// Theme: Wind & Rain
// - Pending tasks are like wind - unsettled, in motion
// - Completing a task is like rain falling - washing away the challenge
// - When all tasks are done, the sky clears

mod application;
mod domain;
mod infrastructure;
mod presentation;

use gpui::*;
use presentation::views::TaskListView;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(420.0), px(680.0)), cx);
        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Waloyo - We Overcome".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| TaskListView::new(cx)),
        );
    });
}
