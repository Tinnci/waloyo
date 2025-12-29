# Waloyo 代码风格指南

本文档定义 Waloyo 项目的代码风格和约定。

## 基本原则

1. **代码正确性和清晰度优先**，速度和效率是次要考虑
2. **不写总结性注释**，注释只用于解释"为什么"这样写
3. **优先在现有文件中实现功能**，除非是全新的逻辑组件
4. **避免使用可能 panic 的函数**，如 `unwrap()`，使用 `?` 传播错误

## Rust 代码风格

### 命名约定

```rust
// 使用完整单词，不使用缩写
let task_queue = Vec::new();  // ✓ 正确
let q = Vec::new();           // ✗ 错误：不要用缩写

// 类型名使用 PascalCase
struct TaskItem { }
enum TaskState { Pending, Completing, Done }

// 函数和变量使用 snake_case
fn handle_task_click() { }
let task_id = TaskId::new();

// 常量使用 SCREAMING_SNAKE_CASE
const PADDING_MD: f32 = 16.0;
```

### 错误处理

```rust
// ✓ 正确：使用 ? 传播错误
fn process_task(id: TaskId) -> Result<Task> {
    let task = find_task(id)?;
    Ok(task)
}

// ✗ 错误：不要使用 unwrap
fn process_task(id: TaskId) -> Task {
    find_task(id).unwrap()  // 危险！
}

// ✗ 错误：不要静默丢弃错误
let _ = risky_operation();  // 危险！

// ✓ 正确：如果必须忽略，记录日志
if let Err(e) = risky_operation() {
    log::error!("Operation failed: {}", e);
}
```

### 异步上下文中的变量遮蔽

在异步上下文中使用变量遮蔽来限定 clone 的作用域：

```rust
// ✓ 正确的模式
executor.spawn({
    let task_ran = task_ran.clone();  // 在闭包内部遮蔽
    async move {
        *task_ran.borrow_mut() = true;
    }
});
```

### 文件组织

```rust
// 不要在路径中创建 mod.rs
// ✗ src/domain/mod.rs       <- 避免
// ✓ src/domain.rs           <- 首选（如果内容简单）

// 对于复杂模块，优先使用显式的库路径在 Cargo.toml 中指定
```

## GPUI 特定约定

### 组件结构

```rust
// 一次性组件使用 RenderOnce
#[derive(IntoElement)]
pub struct Button {
    label: SharedString,
    on_click: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // ...
    }
}

// 有状态视图使用 Render
pub struct TaskListView {
    state: AppState,
}

impl Render for TaskListView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // ...
    }
}
```

### 样式链式调用

```rust
// 保持合理的缩进和分组
div()
    // 布局
    .flex()
    .flex_col()
    .gap_2()
    // 尺寸
    .w_full()
    .h_px(48.0)
    // 外观
    .bg(Theme::surface())
    .rounded(px(8.0))
    // 交互
    .cursor_pointer()
    .hover(|s| s.bg(Theme::surface_hover()))
    // 子元素
    .child(content)
```

### 条件渲染

```rust
// 使用 when 进行条件样式
div()
    .text_color(Theme::text_primary())
    .when(is_completed, |this| this.line_through())

// 使用 when_some 处理 Option
div()
    .when_some(icon, |this, icon| this.child(icon))
```

## 主题系统约定

### 颜色使用

```rust
// 始终使用 Theme 常量，不要直接写颜色值
div().bg(Theme::surface())        // ✓ 正确
div().bg(rgb(0x24283b))           // ✗ 错误

// 语义化颜色命名
Theme::text_primary()     // 主要文本
Theme::text_secondary()   // 次要文本
Theme::state_pending()    // 待办状态
Theme::state_done()       // 完成状态
```

### 间距使用

```rust
// 使用主题中定义的间距常量
.px(px(Theme::PADDING_MD))   // ✓ 正确
.px(px(16.0))                // ✗ 避免魔数
```

## Git 提交约定

```
feat: 添加新功能
fix: 修复 bug
refactor: 重构代码
docs: 更新文档
style: 代码格式调整
chore: 构建/工具相关
```

示例：
```
feat: 实现 TaskItem 组件的点击交互
fix: 修复任务状态切换时的重绘问题
docs: 添加架构设计文档
```

## 测试约定

### GPUI 测试中的定时器

在 GPUI 测试中优先使用 GPUI executor 的定时器：

```rust
// ✓ 正确：使用 GPUI 的 timer
cx.background_executor().timer(duration).await

// ✗ 避免：可能与 run_until_parked() 不兼容
smol::Timer::after(duration).await
```
