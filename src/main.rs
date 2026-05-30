#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // 隐藏运行时的黑框

mod app;
mod fonts;
mod todo;

use eframe::egui;

fn main() {
    let icon = app::load_icon();
    let native_options = eframe::NativeOptions {
        // 设置窗口最小值
        viewport: egui::ViewportBuilder::default()
            .with_icon(icon) // 更换默认图标
            .with_min_inner_size(egui::vec2(500.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Todo",
        native_options,
        Box::new(|cc| Ok(Box::new(app::MyApp::new(cc)))),
    );
}
