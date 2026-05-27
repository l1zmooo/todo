# Todo

> **[中文说明](README.zh.md)**

A lightweight cross-platform desktop todo app built with [egui](https://github.com/emilk/egui) (Rust GUI framework).

| Start fresh | Add and manage tasks |
|---|---|
| _screenshot goes here_ | _screenshot goes here_ |

## Features

- **Add todos** — Type your task, press Enter, done
- **Mark completed** — Toggle `□` / `■` to check off tasks
- **Delete** — Remove individual tasks with one click
- **Auto-save on exit** — All tasks are saved automatically when you close the window
- **Auto-load on startup** — Your previously saved todos are restored when you open the app
- **Chinese font support** — Renders CJK characters beautifully out of the box
- **Minimal UI** — Clean and distraction-free

## Quick Start

### Prerequisites

- Rust (edition 2024, stable toolchain)
- Cargo

### Build & Run

```bash
git clone git@github.com:l1zmooo/todo.git
cd Rust_todo
cargo run --release
```

The binary will be at `target/release/Todo.exe` (Windows) or `target/release/Todo` (Linux/macOS). On Windows in release mode, the console window is automatically hidden.

## Usage

1. Launch the app
2. Type a task in the input box and press **Enter** to add it
3. Click `□` to mark a task as done, `■` to unmark
4. Click **删除** (Delete) to remove a task
5. Close the window — everything is saved automatically

Data is stored in `save.todo` next to the executable. Delete this file to start fresh.

## Dependencies

| Crate | Version | Purpose |
|---|---|---|
| [eframe](https://crates.io/crates/eframe) | 0.34.2 | Native desktop window + egui renderer |
| [egui](https://crates.io/crates/egui) | _(bundled with eframe)_ | Immediate-mode GUI toolkit |

## Project Structure

```
Rust_todo/
├── Cargo.toml
├── assets/
│   └── NotoSerifCJKsc-Black.otf   # Chinese font (built into binary)
├── src/
│   ├── main.rs                     # Entry point & window config
│   ├── app.rs                      # Core UI logic, save/load
│   ├── todo.rs                     # MyTodo struct definition
│   └── fonts.rs                    # Font loading
└── save.todo                       # Auto-generated save file (gitignored)
```

## License

This project is open source. Feel free to use, modify, and share.
