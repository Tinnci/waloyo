# Waloyo - We Overcome

一个基于 GPUI 构建的任务管理应用，采用 "Wind & Rain" (风与雨) 的设计隐喻。

## 项目理念

**Waloyo** 源自兰戈语（Lango），意为 "我们战胜"。每一个待办任务都是需要被克服的挑战。

### 设计隐喻：Wind & Rain (风与雨)

- **待办任务 = 风 (Wind)**：未完成的任务如风般不安定、持续流动
- **完成动作 = 雨 (Rain)**：点击完成时，任务像雨滴一样降落、消散
- **全部完成 = 晴空 (Clear Sky)**：当所有任务都被"战胜"后，天空放晴

## 技术栈

- **语言**: Rust
- **UI 框架**: [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui) (Zed 编辑器的 UI 框架)
- **架构**: Domain-Driven Design (DDD)

## 项目结构

```
waloyo_app/
├── src/
│   ├── main.rs                 # 应用入口
│   ├── domain/                 # 领域层 - 核心业务逻辑
│   │   ├── mod.rs
│   │   └── task.rs             # Task 实体 & TaskState
│   ├── application/            # 应用层 - 用例
│   │   ├── mod.rs
│   │   └── task_service.rs     # 任务管理服务
│   └── presentation/           # 表示层 - UI
│       ├── mod.rs
│       ├── theme.rs            # Wind & Rain 主题配色
│       ├── components/         # 可复用组件
│       │   ├── mod.rs
│       │   └── task_item.rs    # TaskItem 组件
│       └── views/              # 视图
│           ├── mod.rs
│           └── task_list.rs    # 主任务列表视图
├── docs/                       # 文档
│   ├── ARCHITECTURE.md         # 架构设计
│   ├── STYLE_GUIDE.md          # 代码风格指南
│   └── DESIGN.md               # UI/UX 设计指南
└── Cargo.toml
```

## 快速开始

### 前置要求

- Rust toolchain (rustup)
- Windows/macOS/Linux with Vulkan/Metal support

### 编译运行

```bash
cd waloyo_app
cargo run
```

## 文档

- [架构设计](docs/ARCHITECTURE.md)
- [代码风格指南](docs/STYLE_GUIDE.md)
- [UI/UX 设计指南](docs/DESIGN.md)

## 开发路线图

详见 [ROADMAP.md](ROADMAP.md) 查看完整规划。

### 当前进度 (v0.1.0)

- [x] 基础 DDD 架构
- [x] Task 领域模型
- [x] TaskItem 组件
- [x] TaskListView 视图
- [x] Wind & Rain 主题
- [x] Rain Drop 完成动画 ✨
- [x] 任务输入功能 ✨
- [ ] Wind Sway 待办动画
- [ ] Clear Sky 庆祝效果
- [ ] 任务持久化
- [ ] 任务删除

## License

MIT
