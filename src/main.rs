use eframe::egui;
use sysinfo::{System, ProcessesToUpdate};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[cfg(windows)]
use windows::Win32::UI::Shell::ShellExecuteW;
#[cfg(windows)]
use windows::Win32::Foundation::HWND;
#[cfg(windows)]
use windows::core::{w, PCWSTR};

// Constants cho maintainability
const WINDOW_WIDTH: f32 = 400.0;
const WINDOW_HEIGHT: f32 = 550.0;
const UPDATE_INTERVAL_SECS: u64 = 2;
const ROBLOX_PROCESS_NAME: &str = "roblox";
const LAUNCHER_PROCESS_NAME: &str = "launcher";

struct RobloxBooster {
    system: Arc<Mutex<System>>,
    auto_boost: bool,
    roblox_running: bool,
    last_update: Instant,
    status_message: String,
}

impl Default for RobloxBooster {
    fn default() -> Self {
        Self::new()
    }
}

impl RobloxBooster {
    fn new() -> Self {
        Self {
            system: Arc::new(Mutex::new(System::new_all())),
            auto_boost: false,
            roblox_running: false,
            last_update: Instant::now(),
            status_message: String::from("Sẵn sàng"),
        }
    }

    fn update_status(&mut self) {
        if self.last_update.elapsed() <= Duration::from_secs(UPDATE_INTERVAL_SECS) {
            return;
        }

        let mut sys = self.system.lock().unwrap();
        sys.refresh_processes(ProcessesToUpdate::All, true);

        // Kiểm tra Roblox có đang chạy không - sử dụng iterator
        self.roblox_running = sys
            .processes()
            .values()
            .any(|process| {
                let name = process.name().to_string_lossy().to_lowercase();
                name.contains(ROBLOX_PROCESS_NAME) && !name.contains(LAUNCHER_PROCESS_NAME)
            });

        self.last_update = Instant::now();
    }

    #[cfg(windows)]
    fn launch_roblox(&mut self) {
        use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;

        let launch_attempts = [
            w!("roblox://"),
            w!("shell:AppsFolder\\ROBLOXCORPORATION.ROBLOX_55nm5eh3cm0pr!ROBLOX"),
        ];

        for uri in &launch_attempts {
            unsafe {
                let result = ShellExecuteW(
                    HWND::default(),
                    w!("open"),
                    *uri,
                    PCWSTR::null(),
                    PCWSTR::null(),
                    SW_SHOW,
                );

                if result.0 as i32 > 32 {
                    self.status_message = String::from("✓ Đã khởi chạy Roblox");
                    return;
                }
            }
        }

        self.status_message = String::from("✗ Không thể khởi chạy Roblox (Kiểm tra đã cài đặt chưa)");
    }

    #[cfg(not(windows))]
    fn launch_roblox(&mut self) {
        self.status_message = String::from("✗ Launch chỉ hỗ trợ trên Windows");
    }

    fn toggle_auto_boost(&mut self) {
        self.auto_boost = !self.auto_boost;
        self.status_message = if self.auto_boost {
            String::from("✓ Auto Boost: BẬT")
        } else {
            String::from("○ Auto Boost: TẮT")
        };
    }

    #[cfg(windows)]
    fn apply_boost(&mut self) {
        if !self.roblox_running {
            return;
        }

        use windows::Win32::System::Threading::{
            SetPriorityClass, OpenProcess, PROCESS_SET_INFORMATION, HIGH_PRIORITY_CLASS,
        };
        use windows::Win32::System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, Process32Next,
            PROCESSENTRY32, TH32CS_SNAPPROCESS,
        };

        unsafe {
            let snapshot = match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
                Ok(s) => s,
                Err(_) => return,
            };

            let mut entry = PROCESSENTRY32::default();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

            if Process32First(snapshot, &mut entry).is_err() {
                return;
            }

            loop {
                let exe_name = String::from_utf8_lossy(
                    &entry.szExeFile
                        .iter()
                        .take_while(|&&c| c != 0)
                        .map(|&c| c as u8)
                        .collect::<Vec<u8>>()
                ).to_lowercase();

                if exe_name.contains(ROBLOX_PROCESS_NAME) && !exe_name.contains(LAUNCHER_PROCESS_NAME) {
                    if let Ok(handle) = OpenProcess(
                        PROCESS_SET_INFORMATION,
                        false,
                        entry.th32ProcessID,
                    ) {
                        let _ = SetPriorityClass(handle, HIGH_PRIORITY_CLASS);
                    }
                }

                if Process32Next(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }
    }

    #[cfg(not(windows))]
    fn apply_boost(&mut self) {
        // No-op on non-Windows
    }

    fn render_status_indicator(&self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(30, 30, 40))
            .rounding(egui::Rounding::same(8.0))
            .inner_margin(egui::Margin::same(15.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (indicator, text, color) = if self.roblox_running {
                        ("●", "Roblox đang chạy", egui::Color32::GREEN)
                    } else {
                        ("○", "Roblox chưa chạy", egui::Color32::GRAY)
                    };

                    ui.label(
                        egui::RichText::new(indicator)
                            .size(20.0)
                            .color(color)
                    );
                    ui.label(
                        egui::RichText::new(text)
                            .size(16.0)
                            .color(if self.roblox_running { egui::Color32::WHITE } else { color })
                    );
                });
            });
    }

    fn render_button(&self, text: &str, color: egui::Color32) -> egui::Button<'_> {
        egui::Button::new(
            egui::RichText::new(text)
                .size(18.0)
                .strong()
        )
        .fill(color)
        .min_size(egui::vec2(280.0, 60.0))
        .rounding(egui::Rounding::same(10.0))
    }
}

impl eframe::App for RobloxBooster {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_status();

        // Auto boost nếu được bật
        if self.auto_boost && self.roblox_running {
            self.apply_boost();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                
                // Logo/Title
                ui.heading(
                    egui::RichText::new("🚀 ROBLOX BOOSTER")
                        .size(32.0)
                        .strong()
                );
                
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("Tối ưu hiệu suất Roblox")
                        .size(14.0)
                        .color(egui::Color32::GRAY)
                );
                
                ui.add_space(40.0);

                // Status indicator
                self.render_status_indicator(ui);

                ui.add_space(30.0);

                // Auto Boost Button
                let auto_boost_text = if self.auto_boost {
                    "🔥 AUTO BOOST: BẬT"
                } else {
                    "⚪ AUTO BOOST: TẮT"
                };

                let auto_boost_color = if self.auto_boost {
                    egui::Color32::from_rgb(46, 204, 113)
                } else {
                    egui::Color32::from_rgb(149, 165, 166)
                };

                if ui.add(self.render_button(auto_boost_text, auto_boost_color)).clicked() {
                    self.toggle_auto_boost();
                }

                ui.add_space(15.0);

                // Launch Roblox Button
                if ui.add(
                    self.render_button("🎮 KHỞI CHẠY ROBLOX", egui::Color32::from_rgb(52, 152, 219))
                ).clicked() {
                    self.launch_roblox();
                }

                ui.add_space(40.0);

                // Status message
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(44, 62, 80))
                    .rounding(egui::Rounding::same(6.0))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(&self.status_message)
                                .size(13.0)
                                .color(egui::Color32::WHITE)
                        );
                    });

                ui.add_space(30.0);

                // Info footer
                ui.separator();
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("ℹ️ Auto Boost tự động tối ưu khi phát hiện Roblox")
                        .size(11.0)
                        .color(egui::Color32::GRAY)
                );
                ui.label(
                    egui::RichText::new("Made with ❤️ using Rust 🦀")
                        .size(11.0)
                        .color(egui::Color32::GRAY)
                );
            });
        });

        // Request repaint
        ctx.request_repaint_after(Duration::from_secs(UPDATE_INTERVAL_SECS));
    }
}

fn load_icon() -> egui::IconData {
    // Tạo icon 32x32 với màu gradient (xanh dương -> xanh lá)
    let size = 32;
    let mut rgba = Vec::with_capacity(size * size * 4);
    
    for y in 0..size {
        for x in 0..size {
            let center_x = size as f32 / 2.0;
            let center_y = size as f32 / 2.0;
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let dist = (dx * dx + dy * dy).sqrt();
            let max_dist = center_x;
            
            if dist < max_dist - 2.0 {
                // Gradient từ xanh dương đến xanh lá
                let t = y as f32 / size as f32;
                let r = (52.0 * (1.0 - t) + 46.0 * t) as u8;
                let g = (152.0 * (1.0 - t) + 204.0 * t) as u8;
                let b = (219.0 * (1.0 - t) + 113.0 * t) as u8;
                rgba.extend_from_slice(&[r, g, b, 255]);
            } else if dist < max_dist {
                // Border trắng
                rgba.extend_from_slice(&[255, 255, 255, 255]);
            } else {
                // Transparent
                rgba.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
    
    egui::IconData {
        rgba,
        width: size as u32,
        height: size as u32,
    }
}

fn main() -> Result<(), eframe::Error> {
    let icon = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_resizable(false)
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "Roblox Booster",
        options,
        Box::new(|_cc| Ok(Box::new(RobloxBooster::default()))),
    )
}