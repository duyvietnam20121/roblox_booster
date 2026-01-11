use crate::{booster::SystemBooster, config::Config};
use eframe::egui;
use std::time::{Duration, Instant};

// UI Constants
const AUTO_DETECT_INTERVAL: Duration = Duration::from_secs(2);
const BUTTON_SIZE: egui::Vec2 = egui::vec2(240.0, 50.0);
const SMALL_BUTTON_SIZE: egui::Vec2 = egui::vec2(240.0, 35.0);

/// Main application state
pub struct RobloxBoosterApp {
    booster: SystemBooster,
    config: Config,
    booster_enabled: bool,
    status_message: String,
    roblox_count: usize,
    last_check: Instant,
    show_settings: bool,
}

impl RobloxBoosterApp {
    /// Create new application instance
    #[must_use]
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = Config::load();
        let auto_start = config.auto_start_booster;
        let mut booster = SystemBooster::new(config.clone());

        // Auto-start if configured
        let (status_message, booster_enabled) = if auto_start {
            match booster.enable() {
                Ok(msg) => (msg, true),
                Err(e) => (format!("❌ Auto-start failed: {e}"), false),
            }
        } else {
            (String::from("✓ Ready - Click to enable booster"), false)
        };

        Self {
            booster,
            config,
            booster_enabled,
            status_message,
            roblox_count: 0,
            last_check: Instant::now(),
            show_settings: false,
        }
    }

    /// Toggle booster on/off
    fn toggle_booster(&mut self) {
        self.booster_enabled = !self.booster_enabled;

        if self.booster_enabled {
            match self.booster.enable() {
                Ok(msg) => {
                    self.status_message = msg;
                    self.roblox_count = self.booster.get_roblox_process_count();
                }
                Err(e) => {
                    self.status_message = format!("❌ Error: {e}");
                    self.booster_enabled = false;
                }
            }
        } else {
            match self.booster.disable() {
                Ok(msg) => {
                    self.status_message = msg;
                    self.roblox_count = 0;
                }
                Err(e) => {
                    self.status_message = format!("⚠️ Warning: {e}");
                }
            }
        }
    }

    /// Launch Roblox via protocol handler
    fn launch_roblox(&mut self) {
        match self.booster.launch_roblox() {
            Ok(()) => {
                self.status_message = String::from("🚀 Launching Roblox...");
            }
            Err(e) => {
                self.status_message = format!("❌ Launch failed: {e}");
            }
        }
    }

    /// Render main UI
    fn render_main_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(5.0);
            ui.heading(egui::RichText::new("🚀 Roblox Booster").size(18.0));
            ui.label(
                egui::RichText::new("Safe CPU Priority Optimizer")
                    .size(11.0)
                    .color(egui::Color32::GRAY),
            );
        });

        ui.add_space(15.0);
        ui.separator();
        ui.add_space(10.0);

        self.render_status(ui);

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(15.0);

        self.render_controls(ui);

        ui.add_space(15.0);
        ui.separator();
        ui.add_space(8.0);

        self.render_footer(ui);
    }

    /// Render status section
    fn render_status(&self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_min_width(380.0);

            // Status indicator
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Status:").strong());
                let (color, text) = if self.booster_enabled {
                    (egui::Color32::from_rgb(46, 204, 113), "● ACTIVE")
                } else {
                    (egui::Color32::GRAY, "○ INACTIVE")
                };
                ui.colored_label(color, egui::RichText::new(text).strong());
            });

            ui.add_space(5.0);

            // Process count and stats (only when active)
            if self.booster_enabled {
                ui.horizontal(|ui| {
                    ui.label("Roblox processes:");
                    ui.colored_label(
                        egui::Color32::from_rgb(52, 152, 219),
                        self.roblox_count.to_string(),
                    );
                });

                let stats = self.booster.get_stats();
                if stats.processes_boosted > 0 {
                    ui.horizontal(|ui| {
                        ui.label("Optimized:");
                        ui.colored_label(
                            egui::Color32::from_rgb(46, 204, 113),
                            format!("{} process(es)", stats.processes_boosted),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("Priority:");
                        let priority_text = self.config.priority_name();
                        ui.colored_label(
                            egui::Color32::from_rgb(241, 196, 15),
                            priority_text,
                        );
                    });
                }

                ui.add_space(5.0);
            }

            // Status message
            ui.separator();
            ui.add_space(3.0);

            egui::ScrollArea::vertical()
                .max_height(80.0)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(&self.status_message)
                            .size(11.0)
                            .color(egui::Color32::LIGHT_GRAY),
                    );
                });
        });
    }

    /// Render control buttons
    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            // Main toggle button
            let (button_text, button_color) = if self.booster_enabled {
                (
                    "🟢 BOOSTER: ON",
                    egui::Color32::from_rgb(46, 204, 113),
                )
            } else {
                ("⚪ BOOSTER: OFF", egui::Color32::from_rgb(149, 165, 166))
            };

            let button = egui::Button::new(
                egui::RichText::new(button_text)
                    .size(16.0)
                    .color(egui::Color32::WHITE)
                    .strong(),
            )
            .fill(button_color)
            .min_size(BUTTON_SIZE);

            if ui.add(button).clicked() {
                self.toggle_booster();
            }

            ui.add_space(12.0);

            // Launch Roblox button
            let launch_button = egui::Button::new(
                egui::RichText::new("🎮 Launch Roblox")
                    .size(14.0)
                    .color(egui::Color32::WHITE),
            )
            .fill(egui::Color32::from_rgb(52, 152, 219))
            .min_size(SMALL_BUTTON_SIZE);

            if ui.add(launch_button).clicked() {
                self.launch_roblox();
            }

            ui.add_space(12.0);

            // Settings button
            let settings_button = egui::Button::new(
                egui::RichText::new("⚙️ Settings")
                    .size(13.0)
                    .color(egui::Color32::WHITE),
            )
            .fill(egui::Color32::from_rgb(127, 140, 141))
            .min_size(SMALL_BUTTON_SIZE);

            if ui.add(settings_button).clicked() {
                self.show_settings = !self.show_settings;
            }
        });
    }

    /// Render footer info
    fn render_footer(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("🛡️ Safe Mode: Path-validated, Priority-only")
                    .size(10.0)
                    .color(egui::Color32::DARK_GRAY),
            );
            ui.label(
                egui::RichText::new("No memory modification · No code injection")
                    .size(10.0)
                    .color(egui::Color32::DARK_GRAY),
            );
        });
    }

    /// Render settings window
    fn render_settings_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("⚙️ Booster Settings")
            .collapsible(false)
            .resizable(false)
            .default_width(320.0)
            .show(ctx, |ui| {
                let mut config_changed = false;

                ui.heading("Configuration");
                ui.add_space(10.0);

                // Auto-start checkbox
                config_changed |= ui
                    .checkbox(
                        &mut self.config.auto_start_booster,
                        "🚀 Auto-start booster on launch",
                    )
                    .on_hover_text("Automatically enable booster when app starts")
                    .changed();

                ui.add_space(5.0);

                // Auto-detect checkbox
                config_changed |= ui
                    .checkbox(&mut self.config.auto_detect_roblox, "🔍 Auto-detect Roblox")
                    .on_hover_text("Automatically optimize Roblox when detected")
                    .changed();

                ui.add_space(5.0);

                // Memory optimization (placeholder)
                config_changed |= ui
                    .checkbox(&mut self.config.clear_memory_cache, "💾 Memory optimization")
                    .on_hover_text("Enable memory optimization (safe mode)")
                    .changed();

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // Priority level selector
                ui.label(egui::RichText::new("Process Priority Level:").strong());
                ui.add_space(5.0);

                let mut priority = self.config.priority_level as usize;

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        config_changed |= ui
                            .radio_value(&mut priority, 0, "⚪ Normal")
                            .on_hover_text("Standard priority (safest, minimal boost)")
                            .changed();

                        config_changed |= ui
                            .radio_value(&mut priority, 1, "🟡 Above Normal")
                            .on_hover_text("Higher priority (recommended, balanced)")
                            .changed();

                        config_changed |= ui
                            .radio_value(&mut priority, 2, "🔴 High")
                            .on_hover_text("Highest priority (maximum boost, may affect other apps)")
                            .changed();
                    });
                });

                self.config.priority_level = priority as u8;

                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);

                // Action buttons
                ui.horizontal(|ui| {
                    // Save button
                    let save_button = egui::Button::new(
                        egui::RichText::new("💾 Save").color(egui::Color32::WHITE),
                    )
                    .fill(egui::Color32::from_rgb(46, 204, 113));

                    if ui.add(save_button).clicked() {
                        match self.config.save() {
                            Ok(()) => {
                                self.booster.update_config(self.config.clone());
                                self.status_message = String::from("✓ Settings saved successfully");
                                self.show_settings = false;
                            }
                            Err(e) => {
                                self.status_message = format!("❌ Save failed: {e}");
                            }
                        }
                    }

                    ui.add_space(5.0);

                    // Cancel button
                    let cancel_button = egui::Button::new(
                        egui::RichText::new("❌ Cancel").color(egui::Color32::WHITE),
                    )
                    .fill(egui::Color32::from_rgb(231, 76, 60));

                    if ui.add(cancel_button).clicked() {
                        self.config = Config::load();
                        self.show_settings = false;
                    }

                    ui.add_space(10.0);

                    // Unsaved indicator
                    if config_changed {
                        ui.colored_label(
                            egui::Color32::from_rgb(241, 196, 15),
                            egui::RichText::new("⚠ Unsaved changes").strong(),
                        );
                    }
                });
            });
    }
}

impl eframe::App for RobloxBoosterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-detect with interval
        if self.booster_enabled && self.last_check.elapsed() > AUTO_DETECT_INTERVAL {
            self.last_check = Instant::now();

            if let Some(msg) = self.booster.auto_detect_and_boost() {
                self.status_message = msg;
            }

            self.roblox_count = self.booster.get_roblox_process_count();
        }

        // Request repaint for smooth animations
        ctx.request_repaint_after(AUTO_DETECT_INTERVAL);

        // Render main panel
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_ui(ui);
        });

        // Render settings window if open
        if self.show_settings {
            self.render_settings_window(ctx);
        }
    }
}