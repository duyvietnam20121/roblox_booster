use crate::{booster::Booster, config::Config};
use eframe::egui;

pub struct BoosterApp {
    config: Config,
    booster: Booster,
    status_message: String,
    show_settings: bool,
    last_check: std::time::Instant,
}

impl BoosterApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        Self {
            config,
            booster: Booster::new(),
            status_message: String::from("Chưa kiểm tra"),
            show_settings: false,
            last_check: std::time::Instant::now(),
        }
    }

    fn render_main_controls(&mut self, ui: &mut egui::Ui) {
        ui.heading("🚀 Roblox Booster");
        ui.add_space(10.0);

        // Toggle Auto Boost
        let auto_boost_text = if self.config.auto_boost {
            "✅ Auto Boost: ON"
        } else {
            "❌ Auto Boost: OFF"
        };

        if ui
            .add_sized(
                [200.0, 40.0],
                egui::Button::new(auto_boost_text).fill(if self.config.auto_boost {
                    egui::Color32::from_rgb(0, 150, 0)
                } else {
                    egui::Color32::from_rgb(150, 0, 0)
                }),
            )
            .clicked()
        {
            self.config.auto_boost = !self.config.auto_boost;
            self.config.save().ok();
        }

        ui.add_space(10.0);

        // Manual Boost Button
        if ui
            .add_sized([200.0, 30.0], egui::Button::new("⚡ Boost Ngay"))
            .clicked()
        {
            match self.booster.boost_roblox(self.config.optimization_level) {
                Ok(msg) => self.status_message = msg,
                Err(e) => self.status_message = format!("❌ Lỗi: {e}"),
            }
        }

        ui.add_space(10.0);

        // Status
        ui.label(format!("📊 Trạng thái: {}", self.status_message));

        let roblox_status = if self.booster.is_roblox_running() {
            "✅ Roblox đang chạy"
        } else {
            "⚠️ Roblox chưa chạy"
        };
        ui.label(roblox_status);
    }

    fn render_settings_window(&mut self, ctx: &egui::Context) {
        let mut show_settings = self.show_settings;
        let mut should_close = false;

        egui::Window::new("⚙️ Settings")
            .open(&mut show_settings)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Optimization Level");
                ui.add_space(5.0);

                use crate::config::OptimizationLevel;

                ui.radio_value(
                    &mut self.config.optimization_level,
                    OptimizationLevel::Low,
                    "🔵 Low (CPU Priority)",
                );
                ui.radio_value(
                    &mut self.config.optimization_level,
                    OptimizationLevel::Medium,
                    "🟡 Medium (CPU + Memory)",
                );
                ui.radio_value(
                    &mut self.config.optimization_level,
                    OptimizationLevel::High,
                    "🔴 High (Maximum)",
                );

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);

                ui.checkbox(
                    &mut self.config.auto_detect_roblox,
                    "Auto-detect Roblox process",
                );

                ui.add_space(10.0);

                if ui.button("💾 Lưu").clicked() {
                    if let Err(e) = self.config.save() {
                        self.status_message = format!("❌ Lỗi lưu config: {e}");
                    } else {
                        self.status_message = "✅ Đã lưu settings".to_string();
                    }
                    should_close = true;
                }
            });

        // Update show_settings based on window state and close button
        self.show_settings = show_settings && !should_close;
    }
}

impl eframe::App for BoosterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-boost logic (check mỗi 5s)
        if self.config.auto_boost && self.last_check.elapsed().as_secs() >= 5 {
            if self.booster.is_roblox_running() {
                if let Ok(msg) = self.booster.boost_roblox(self.config.optimization_level) {
                    self.status_message = format!("🔄 {msg}");
                }
            }
            self.last_check = std::time::Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                self.render_main_controls(ui);

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);

                // Settings button
                if ui.button("⚙️ Settings").clicked() {
                    self.show_settings = true;
                }
            });
        });

        // Render settings window if open
        if self.show_settings {
            self.render_settings_window(ctx);
        }

        // Request repaint cho auto-boost
        if self.config.auto_boost {
            ctx.request_repaint_after(std::time::Duration::from_secs(1));
        }
    }
}
