use crate::{booster::Booster, config::Config};
use eframe::egui;

pub struct BoosterApp {
    config: Config,
    optimizer: Booster,
    status_message: String,
    show_settings: bool,
    last_optimization_check: std::time::Instant,
}

impl Drop for BoosterApp {
    fn drop(&mut self) {
        // Cleanup: Reset priority when app closes
        #[cfg(target_os = "windows")]
        if let Err(e) = self.optimizer.reset() {
            eprintln!("Cleanup warning: {e}");
        }
    }
}

impl BoosterApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        Self {
            config,
            optimizer: Booster::new(),
            status_message: String::from("Ready"),
            show_settings: false,
            last_optimization_check: std::time::Instant::now(),
        }
    }

    fn render_main_controls(&mut self, ui: &mut egui::Ui) {
        ui.heading("🚀 Roblox Performance Optimizer");
        ui.add_space(10.0);

        // Auto optimization toggle
        let auto_text = if self.config.auto_boost {
            "✅ Auto Optimization: ON"
        } else {
            "❌ Auto Optimization: OFF"
        };

        if ui
            .add_sized(
                [250.0, 40.0],
                egui::Button::new(auto_text).fill(if self.config.auto_boost {
                    egui::Color32::from_rgb(0, 150, 0)
                } else {
                    egui::Color32::from_rgb(100, 100, 100)
                }),
            )
            .clicked()
        {
            self.config.auto_boost = !self.config.auto_boost;
            self.config.save().ok();
        }

        ui.add_space(10.0);

        // Manual optimization button
        if ui
            .add_sized([250.0, 35.0], egui::Button::new("⚡ Optimize Now"))
            .clicked()
        {
            match self.optimizer.optimize(self.config.optimization_level) {
                Ok(msg) => self.status_message = msg,
                Err(e) => self.status_message = format!("Error: {e}"),
            }
        }

        ui.add_space(15.0);

        // Status display
        ui.group(|ui| {
            ui.label(format!("📊 Status: {}", self.status_message));

            #[cfg(target_os = "windows")]
            if let Some(info) = self.optimizer.get_roblox_info() {
                ui.label(format!("✅ Process: {} (PID: {})", info.name, info.pid));
            } else {
                ui.label("⚠️ Roblox not detected");
            }

            #[cfg(not(target_os = "windows"))]
            {
                let status = if self.optimizer.is_roblox_running() {
                    "✅ Roblox is running"
                } else {
                    "⚠️ Roblox not detected"
                };
                ui.label(status);
            }
        });
    }

    fn render_settings_window(&mut self, ctx: &egui::Context) {
        let mut show_settings = self.show_settings;
        let mut should_close = false;

        egui::Window::new("⚙️ Settings")
            .open(&mut show_settings)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Priority Level");
                ui.add_space(5.0);

                use crate::config::OptimizationLevel;

                ui.radio_value(
                    &mut self.config.optimization_level,
                    OptimizationLevel::Low,
                    "🔵 Low - Normal Priority",
                );
                ui.label("    Balanced performance, safe for multitasking");

                ui.add_space(5.0);

                ui.radio_value(
                    &mut self.config.optimization_level,
                    OptimizationLevel::Medium,
                    "🟡 Medium - Above Normal Priority",
                );
                ui.label("    Recommended for gaming");

                ui.add_space(5.0);

                ui.radio_value(
                    &mut self.config.optimization_level,
                    OptimizationLevel::High,
                    "🔴 High - High Priority",
                );
                ui.label("    Maximum performance, may affect other apps");

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);

                ui.checkbox(
                    &mut self.config.auto_detect_roblox,
                    "Auto-detect Roblox process",
                );

                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    if ui.button("💾 Save").clicked() {
                        if let Err(e) = self.config.save() {
                            self.status_message = format!("Save error: {e}");
                        } else {
                            self.status_message = "Settings saved".to_string();
                        }
                        should_close = true;
                    }

                    ui.add_space(10.0);

                    #[cfg(target_os = "windows")]
                    if ui.button("🔄 Reset Priority").clicked() {
                        match self.optimizer.reset() {
                            Ok(()) => self.status_message = "Priority reset to Normal".to_string(),
                            Err(e) => self.status_message = format!("Reset error: {e}"),
                        }
                    }
                });
            });

        self.show_settings = show_settings && !should_close;
    }
}

impl eframe::App for BoosterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-optimization logic (every 5 seconds)
        if self.config.auto_boost
            && self.last_optimization_check.elapsed().as_secs() >= 5
        {
            if self.optimizer.is_roblox_running() {
                if let Ok(msg) = self.optimizer.optimize(self.config.optimization_level) {
                    self.status_message = format!("Auto: {msg}");
                }
            }
            self.last_optimization_check = std::time::Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                self.render_main_controls(ui);

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);

                if ui.button("⚙️ Settings").clicked() {
                    self.show_settings = true;
                }
            });
        });

        if self.show_settings {
            self.render_settings_window(ctx);
        }

        // Request repaint for auto-optimization
        if self.config.auto_boost {
            ctx.request_repaint_after(std::time::Duration::from_secs(1));
        }
    }
}