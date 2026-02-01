mod booster;
mod config;
mod ui;

use iced::{Application, Settings};

fn main() -> iced::Result {
    // Banner
    println!("╔══════════════════════════════════════╗");
    println!("║     ROBLOX BOOSTER v0.1.0            ║");
    println!("║     Tối ưu hóa Roblox cho Windows    ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("Tính năng:");
    println!("  ✓ Timer Resolution (1ms)");
    println!("  ✓ GPU Priority Boost");
    println!("  ✓ Memory Cleanup (60s interval)");
    println!("  ✓ Auto-Detection");
    println!("  ✓ Config Persistence");
    println!("  ✓ CPU Affinity (P-cores)");
    println!();
    println!("⚠️  Khuyến nghị: Chạy với quyền Administrator");
    println!();
    
    ui::RobloxBooster::run(Settings {
        window: iced::window::Settings {
            size: (450, 500),
            resizable: false,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    })
}