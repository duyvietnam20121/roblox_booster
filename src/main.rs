mod booster;
mod config;
mod ui;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 350.0])
            .with_resizable(false)
            .with_icon(load_icon()),
        ..Default::default()
    };
    
    eframe::run_native(
        "Roblox Booster",
        options,
        Box::new(|cc| Box::new(ui::RobloxBoosterApp::new(cc))),
    )
}

fn load_icon() -> egui::IconData {
    // Simple 32x32 icon - green circle
    let (width, height) = (32, 32);
    let mut rgba = vec![0u8; width * height * 4];
    
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4;
            let dx = x as f32 - 16.0;
            let dy = y as f32 - 16.0;
            let dist = (dx * dx + dy * dy).sqrt();
            
            if dist < 14.0 {
                rgba[idx] = 50;      // R
                rgba[idx + 1] = 200; // G
                rgba[idx + 2] = 50;  // B
                rgba[idx + 3] = 255; // A
            }
        }
    }
    
    egui::IconData {
        rgba,
        width: width as u32,
        height: height as u32,
    }
}