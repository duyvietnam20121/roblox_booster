// Ẩn console window khi release trên Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod booster;
mod config;
mod ui;

fn main() {
    // Load config
    let config = config::Config::load();

    // Khởi tạo app
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([350.0, 250.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "Roblox Booster",
        native_options,
        Box::new(|cc| Ok(Box::new(ui::BoosterApp::new(cc, config)))),
    ) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn load_icon() -> egui::IconData {
    // Icon mặc định (64x64 red square)
    let rgba = vec![255u8, 0, 0, 255].repeat(64 * 64);
    egui::IconData {
        rgba,
        width: 64,
        height: 64,
    }
}
