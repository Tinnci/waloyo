# Waloyo 架构设计

本文档描述 Waloyo 应用的架构设计原则和实现细节。

## 架构概述

Waloyo 采用 **Domain-Driven Design (DDD)** 分层架构，将代码组织为三个主要层次：

```
┌─────────────────────────────────────────────────────┐
│                 Presentation Layer                  │
│          (UI Components, Views, Theme)              │
├─────────────────────────────────────────────────────┤
│                 Application Layer                   │
│            (Use Cases, Services)                    │
├─────────────────────────────────────────────────────┤
│                   Domain Layer                      │
│          (Entities, Value Objects)                  │
└─────────────────────────────────────────────────────┘
```

## 层次职责

### 1. Domain Layer (领域层)

**路径**: `src/domain/`

领域层是应用的核心，包含纯粹的业务逻辑。这一层**不依赖任何 UI 或基础设施**。

#### 核心实体

**Task** - 任务实体
```rust
pub struct Task {
    pub id: TaskId,           // 唯一标识
    pub content: SharedString, // 任务内容
    pub state: TaskState,      // 当前状态
    pub created_at: Instant,   // 创建时间
    pub updated_at: Instant,   // 更新时间
}
```

**TaskState** - 任务状态枚举
```rust
pub enum TaskState {
    Pending,     // 待办 - "风" 状态
    Completing,  // 完成中 - "雨滴下落" 动画
    Done,        // 已完成 - "平静" 状态
}
```

#### 设计原则

- 所有业务规则都在领域层中表达
- 不引入任何 GPUI 依赖（除了 `SharedString` 用于性能优化）
- 实体方法应该是纯函数或只修改自身状态

### 2. Application Layer (应用层)

**路径**: `src/application/`

应用层协调领域对象以完成特定的应用用例。

#### 核心服务

**TaskService** - 任务管理服务
```rust
pub struct TaskService {
    tasks: Vec<Task>,
}

impl TaskService {
    pub fn add_task(&mut self, content: impl Into<SharedString>) -> TaskId;
    pub fn begin_completing(&mut self, id: TaskId) -> bool;
    pub fn finish_completing(&mut self, id: TaskId) -> bool;
    pub fn all_overcome(&self) -> bool;
    // ...
}
```

#### 设计原则

- 服务是无状态的协调者（当前实现包含状态，未来可能移到仓储层）
- 不包含 UI 逻辑
- 可以依赖领域层，但不依赖表示层

### 3. Presentation Layer (表示层)

**路径**: `src/presentation/`

表示层处理所有 GPUI 相关的渲染和用户交互。

#### 子模块

- **theme.rs** - 主题和颜色常量
- **components/** - 可复用的 UI 组件
- **views/** - 完整的页面视图

#### 组件设计

**TaskItem** - 使用 `RenderOnce` trait
```rust
#[derive(IntoElement)]
pub struct TaskItem {
    task: Task,
    on_complete: Option<Box<dyn Fn(TaskId, &mut Window, &mut App) + 'static>>,
}

impl RenderOnce for TaskItem {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // ...
    }
}
```

**TaskListView** - 使用 `Render` trait
```rust
pub struct TaskListView {
    task_service: TaskService,
}

impl Render for TaskListView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // ...
    }
}
```

## 数据流

```
User Click
    │
    ▼
┌─────────────────┐
│   TaskItem      │  on_complete callback
│   (Component)   │───────────────────────┐
└─────────────────┘                       │
                                          ▼
                               ┌─────────────────┐
                               │  TaskListView   │
                               │    (View)       │
                               └────────┬────────┘
                                        │ handle_task_click()
                                        ▼
                               ┌─────────────────┐
                               │  TaskService    │
                               │  (Application)  │
                               └────────┬────────┘
                                        │ begin_completing()
                                        ▼
                               ┌─────────────────┐
                               │     Task        │
                               │   (Domain)      │
                               └─────────────────┘
```

## GPUI 特定模式

### Entity 和 WeakEntity

在 GPUI 中，视图状态通过 `Entity<T>` 管理。为避免循环引用，在回调中使用 `WeakEntity<T>`:

```rust
let entity = cx.entity().downgrade();  // 获取弱引用

// 在回调中
entity.update(cx, |view, cx| {
    view.handle_task_click(id, cx);
});
```

### Context 类型

- `App` - 全局应用上下文
- `Context<T>` - 特定实体的上下文，可调用 `notify()` 触发重绘
- `Window` - 窗口上下文

### IntoElement vs Render

- `RenderOnce` - 一次性组件，消费 self
- `Render` - 有状态视图，保持 self 的引用

## 未来扩展点

1. **Repository 层** - 将 `TaskService` 中的状态移出，引入持久化
2. **Event Sourcing** - 任务状态变更通过事件驱动
3. **动画系统** - 在表示层添加专用的动画模块
