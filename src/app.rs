use crate::fonts;
use crate::todo::MyTodo;
use eframe::egui::text_selection::visuals;
use eframe::egui::{self, Color32, Layout};
use egui::RichText;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

// 存储文件路径
static SAVE_PATH: &str = "save.todo";

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
        app.is_dark_theme = true;
        app
    }
}

impl eframe::App for MyApp {
    // 每一帧都会被调用，负责绘制整个 UI
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("顶部面板").show_inside(ui, |ui| {
            // 输入框 逻辑
            // horizontal 可将ui显示在同一行
            ui.horizontal(|ui| {
                ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(16.0);
                    // 使用回车来添加待办
                    let response = ui.text_edit_singleline(&mut self.user_input);
                    // 当输入框失去焦点，并且内容不为空时，添加todo
                    if response.lost_focus() && !self.user_input.trim().is_empty() {
                        self.list.push(MyTodo::new(self.user_input.clone()));
                        self.user_input.clear();
                        response.request_focus(); // 请求焦点，实现再次输入
                    }
                    ui.add_space(8.0);
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(16.0);
                    // 切换主题
                    let visuals = if self.is_dark_theme {
                        egui::Visuals::dark()
                    } else {
                        egui::Visuals::light()
                    };
                    ui.ctx().set_visuals(visuals);
                    let theme_ico = if self.is_dark_theme { "☀" } else { "🌙" };
                    if ui.button(RichText::new(theme_ico).size(15.0)).clicked() {
                        self.is_dark_theme = !self.is_dark_theme;
                    }
                    ui.add_space(8.0);
                });
            });
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // 实现内容超出显示范围时可滚动的效果
            egui::ScrollArea::vertical().show(ui, |ui| {
                // 没有代办时显示
                if self.list.is_empty() {
                    ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        ui.label(RichText::new("当前没有任何代办").size(25.0));
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
                            ui.label(RichText::new(todo.todo_name.clone()).size(15.0));
                            ui.with_layout(Layout::top_down(egui::Align::RIGHT), |ui| {
                                if ui.button(RichText::new("删除").size(15.0)).clicked() {
                                    remove_indices.push(idx);
                                }
                            })
                        });
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

// 保存函数
fn save(my_app: &MyApp) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // 先清空再写，避免 append 导致的重复
        .open(SAVE_PATH)?; // ? 传播错误，路径相对于工作目录

    for todo in &my_app.list {
        let line = format!("{},{}\n", todo.todo_name, todo.is_finish);
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}
/// 从文件读取
fn read_save(my_app: &mut MyApp) -> std::io::Result<()> {
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
    Ok(())
}
