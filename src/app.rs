use crate::fonts;
use crate::todo::MyTodo;
use eframe::egui::{self, Layout, ViewportCommand};
use egui::RichText;
use image;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

// 存储文件路径
static SAVE_PATH: &str = "save.todo";
static THEME_SAVE_PATH: &str = "theme_save.todo";

#[derive(Default)]
pub struct MyApp {
    pub list: Vec<MyTodo>,
    pub user_input: String,
    pub is_dark_theme: bool,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 字体设置（逻辑移至 fonts.rs，注释保留在调用处）
        fonts::setup_fonts(&cc.egui_ctx);
        let mut app = Self::default(); // 创建空实例
        // 实现自动读取已保存的内容
        if let Err(e) = read_save(&mut app) {
            eprintln!("读取时发生错误:{}", e);
        }
        app
    }
}

impl eframe::App for MyApp {
    // 每一帧都会被调用，负责绘制整个 UI
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("顶部面板").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                    let logo = ui.add(
                        egui::Label::new(RichText::new("Todo").size(30.0))
                            .selectable(false)
                            .sense(egui::Sense::drag()), // 捕获鼠标交互事件
                    );
                    // 拖动窗口实现
                    if logo.drag_started() {
                        ui.send_viewport_cmd(ViewportCommand::StartDrag);
                    }
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(5.0);
                    // 关闭按钮
                    if ui.button(RichText::new(" ❌ ").size(20.0)).clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    // 最大化按钮
                    if ui.button(RichText::new(" ⭕ ").size(20.0)).clicked() {
                        let max = ui.ctx().input(|i| i.viewport().maximized).unwrap();
                        ui.send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
                    }
                    // 最小化按钮
                    if ui.button(RichText::new(" ➖ ").size(20.0)).clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                    }
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);
                });
            });
        });
        egui::Panel::bottom("底部面板")
            .max_size(30.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    // 显示未完成的todo数量
                    let mut unfinished_todo: u32 = 0;
                    for i in &self.list {
                        if !i.is_finish {
                            unfinished_todo += 1;
                        }
                    }
                    if unfinished_todo != 0 {
                        ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add(
                                egui::Label::new(
                                    RichText::new(format!("当前有{}条代办未完成", unfinished_todo))
                                        .size(15.0),
                                )
                                .selectable(false),
                            )
                        });
                    }
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(3.0);
                        // 切换主题
                        let mut visuals = if self.is_dark_theme {
                            egui::Visuals::dark()
                        } else {
                            egui::Visuals::light()
                        };
                        // 亮色主题颜色设置
                        if !self.is_dark_theme {
                            visuals.panel_fill = egui::Color32::from_rgb(225, 225, 225);
                        }
                        ui.ctx().set_visuals(visuals); // 渲染主题
                        // 切换主题的按钮图标
                        let theme_ico = if self.is_dark_theme {
                            " ☀ "
                        } else {
                            " 🌙 "
                        };
                        if ui.button(RichText::new(theme_ico).size(17.0)).clicked() {
                            self.is_dark_theme = !self.is_dark_theme;
                        }
                    });
                });
            });
        // 中部面板
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // 实现内容超出显示范围时可滚动的效果
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                    // 输入框 逻辑
                    // 使用回车来添加待办
                    let response = ui.text_edit_singleline(&mut self.user_input);
                    // 当输入框失去焦点，并且内容不为空时，添加todo
                    if response.lost_focus() && !self.user_input.trim().is_empty() {
                        self.list.push(MyTodo::new(self.user_input.clone()));
                        self.user_input.clear();
                        response.request_focus(); // 请求焦点，实现再次输入
                    }
                });
                ui.add_space(5.0);
                // 没有代办时显示
                if self.list.is_empty() {
                    ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                        ui.add_space(30.0);
                        let logo2 = ui.add(
                            egui::Label::new(RichText::new("当前没有任何代办").size(25.0))
                                .selectable(false)
                                .sense(egui::Sense::drag()), // 捕获鼠标交互事件
                        );
                        // 拖动窗口实现
                        if logo2.drag_started() {
                            ui.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        }
                    });
                }
                // 显示未完成的todo
                let mut remove_indices: Vec<usize> = Vec::new(); // 创建删除列表，将 未完成 和 已完成 todo的索引存入，之后一并删除
                for (idx, todo) in &mut self.list.iter_mut().enumerate() {
                    if todo.is_finish == false {
                        // horizontal 可将ui显示在同一行
                        ui.horizontal(|ui| {
                            if ui.button(RichText::new("□").size(15.0)).clicked() {
                                todo.is_finish = true;
                            }
                            ui.add(
                                egui::Label::new(RichText::new(todo.todo_name.clone()).size(15.0))
                                    .selectable(false),
                            );

                            ui.with_layout(Layout::top_down(egui::Align::RIGHT), |ui| {
                                if ui.button(RichText::new("删除").size(15.0)).clicked() {
                                    remove_indices.push(idx);
                                }
                            });
                        });
                        ui.add_space(3.0);
                    };
                }
                // 当有已完成的 todo 时显示标签
                if self.list.iter().any(|t| t.is_finish) {
                    ui.label(RichText::new("已完成：").size(15.0));
                }
                // 显示已完成的todo
                for (idx, todo) in &mut self.list.iter_mut().enumerate() {
                    if todo.is_finish == true {
                        // horizontal 可将ui显示在同一行
                        ui.horizontal(|ui| {
                            if ui.button(RichText::new("■").size(15.0)).clicked() {
                                todo.is_finish = false;
                            }
                            ui.label(RichText::new(todo.todo_name.clone()).size(15.0));
                            ui.with_layout(Layout::top_down(egui::Align::RIGHT), |ui| {
                                if ui.button(RichText::new("删除").size(15.0)).clicked() {
                                    remove_indices.push(idx);
                                }
                            })
                        });
                        ui.add_space(3.0);
                    }
                }
                // 倒序删除，避免索引偏移
                remove_indices.sort_unstable_by(|a, b| b.cmp(a));
                remove_indices.dedup();
                for idx in remove_indices {
                    self.list.remove(idx);
                }
            });
        });
    }
    // 关闭程序时执行
    fn on_exit(&mut self) {
        if let Err(e) = save(self) {
            eprintln!("自动存档时发生错误:{e}");
        }
    }
}

// === 函数定义 ===

// 保存函数
fn save(my_app: &MyApp) -> std::io::Result<()> {
    // todo保存
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // 先清空再写，避免 append 导致的重复
        .open(SAVE_PATH)?; // ? 传播错误，路径相对于工作目录

    for todo in &my_app.list {
        let line = format!("{},{}\n", todo.todo_name, todo.is_finish);
        file.write_all(line.as_bytes())?;
    }

    // 主题保存
    let mut theme = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(THEME_SAVE_PATH)?;

    let theme_state = format!("{}", my_app.is_dark_theme);
    theme.write_all(theme_state.as_bytes())?;

    Ok(())
}
/// 从文件读取
fn read_save(my_app: &mut MyApp) -> std::io::Result<()> {
    // todo读取
    let s = fs::read_to_string(SAVE_PATH)?;
    my_app.list.clear();
    for line in s.lines() {
        if let Some((name, finished)) = line.split_once(',') {
            my_app.list.push(MyTodo {
                todo_name: name.to_string(),
                is_finish: finished == "true",
            });
        }
    }
    // 主题读取
    let t = fs::read_to_string(THEME_SAVE_PATH)?;
    my_app.is_dark_theme = t.trim().parse::<bool>().unwrap_or(true); // 将String转换成bool
    Ok(())
}
// 更换图标函数,需使用ico文件
pub fn load_icon() -> egui::IconData {
    let image = image::load_from_memory(include_bytes!("../assets/futaba.ico"))
        .unwrap()
        .into_rgba8();
    let (width, height) = image.dimensions();
    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}
