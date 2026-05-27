// 字体设置模块
use eframe::egui::{self};
use std::sync::Arc;

pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    // 将字体文件编译进二进制
    let font_data = include_bytes!("../assets/NotoSerifCJKsc-Black.otf").to_vec();
    fonts.font_data.insert(
        "chinese_font".to_owned(),
        Arc::new(egui::FontData::from_owned(font_data)),
    );
    // 替换默认的 proportional 字体
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "chinese_font".to_owned());
    // 可选：同时替换 monospace 字体
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "chinese_font".to_owned());
    ctx.set_fonts(fonts);
}
