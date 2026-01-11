// Hide console window on Windows release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod booster;
mod config;
mod ui;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Configure window options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 380.0])
            .with_resizable(false)
            .with_icon(create_app_icon())
            .with_title("Roblox Booster")
            .with_always_on_top(false),
        ..Default::default()
    };

    // Run application
    eframe::run_native(
        "Roblox Booster",
        options,
        Box::new(|cc| Ok(Box::new(ui::RobloxBoosterApp::new(cc)))),
    )
}

/// Create application icon (32x32 green circle with shadow)
fn create_app_icon() -> egui::IconData {
    const SIZE: usize = 32;
    const CENTER: f32 = 16.0;
    const RADIUS: f32 = 13.0;
    const SHADOW_OFFSET: f32 = 1.0;

    let mut rgba = vec![0u8; SIZE * SIZE * 4];

    for y in 0..SIZE {
        for x in 0..SIZE {
            let idx = (y * SIZE + x) * 4;
            let dx = x as f32 - CENTER;
            let dy = y as f32 - CENTER;
            let dist = (dx * dx + dy * dy).sqrt();

            // Shadow layer (darker, offset)
            let shadow_dx = dx - SHADOW_OFFSET;
            let shadow_dy = dy - SHADOW_OFFSET;
            let shadow_dist = (shadow_dx * shadow_dx + shadow_dy * shadow_dy).sqrt();

            if shadow_dist < RADIUS {
                rgba[idx] = 30; // R
                rgba[idx + 1] = 30; // G
                rgba[idx + 2] = 30; // B
                rgba[idx + 3] = 80; // A (semi-transparent)
            }

            // Main icon layer (green gradient)
            if dist < RADIUS {
                let intensity = 1.0 - (dist / RADIUS * 0.3);
                rgba[idx] = (50.0 * intensity) as u8; // R
                rgba[idx + 1] = (200.0 * intensity) as u8; // G
                rgba[idx + 2] = (50.0 * intensity) as u8; // B
                rgba[idx + 3] = 255; // A (solid)
            }
        }
    }

    egui::IconData {
        rgba,
        width: SIZE as u32,
        height: SIZE as u32,
    }
}