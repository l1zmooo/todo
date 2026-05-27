# Todo

一款轻量级跨平台桌面待办事项应用，基于 [egui](https://github.com/emilk/egui) 构建。

## 功能

- **添加待办** — 输入任务内容，按回车即添加
- **标记完成** — 点击 `□` / `■` 切换待办状态
- **删除任务** — 一键删除不需要的待办
- **退出自动保存** — 关闭窗口时自动保存所有待办到本地文件
- **启动自动读取** — 再次打开应用时自动恢复上次的待办数据
- **中文字体支持** — 内置 Noto Serif CJK SC 字体，中文显示效果优秀
- **极简界面** — 干净清爽，无多余干扰

## 快速开始

### 环境要求

- Rust（edition 2024，stable 工具链）
- Cargo

### 编译运行

```bash
git clone git@github.com:l1zmooo/todo.git
cd Rust_todo
cargo run --release
```

编译产物位于 `target/release/Todo.exe`（Windows）/ `target/release/Todo`（Linux/macOS）。Windows 发布版会自动隐藏控制台窗口。

## 使用说明

1. 启动应用
2. 在输入框中输入待办内容，按 **回车** 添加
3. 点击 `□` 标记已完成，点击 `■` 恢复未完成
4. 点击 **删除** 移除不需要的待办
5. 直接关闭窗口 — 所有数据自动保存

数据保存在运行目录下的 `save.todo` 文件中，删除该文件即可清空所有待办从头开始。

## 依赖

| 包名 | 版本 | 用途 |
|---|---|---|
| [eframe](https://crates.io/crates/eframe) | 0.34.2 | 原生桌面窗口 + egui 渲染 |
| [egui](https://crates.io/crates/egui) | _(随 eframe 引入)_ | 即时模式 GUI 框架 |

## 项目结构

```
Rust_todo/
├── Cargo.toml
├── assets/
│   └── NotoSerifCJKsc-Black.otf   # 中文字体（编译进二进制）
├── src/
│   ├── main.rs                     # 程序入口 & 窗口配置
│   ├── app.rs                      # 核心 UI 逻辑、保存/读取
│   ├── todo.rs                     # MyTodo 结构体定义
│   └── fonts.rs                    # 字体加载
├── README.md                       # 英文说明
├── README.zh.md                    # 中文说明（就是本文）
└── save.todo                       # 自动生成的存档文件（已被 gitignore）
```

## 开源协议

本项目开源，欢迎自由使用、修改和分享。
