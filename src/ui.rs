use crate::{booster::SystemBooster, config::Config};
use eframe::egui;
use std::time::{Duration, Instant};

/// Main application state
pub struct RobloxBoosterApp {
    booster: SystemBooster,
    config: Config,
    is_enabled: bool,
    status_message: String,
    last_auto_check: Instant,
    show_settings: bool,
    temp_config: Config, // Temporary config for editing
    temp_custom_path: String, // Temp path input
}

impl RobloxBoosterApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        let config = Config::load();
        let booster = SystemBooster::new(config.clone());
        
        Self {
            booster,
            config: config.clone(),
            is_enabled: false,
            status_message: "Ready".to_string(),
            last_auto_check: Instant::now(),
            show_settings: false,
            temp_config: config.clone(),
            temp_custom_path: config.custom_roblox_path.clone().unwrap_or_default(),
        }
    }

    fn render_main_ui(&mut self, ui: &mut egui::Ui) {
        // Title with version
        ui.heading(format!("🚀 Roblox Booster v{}", SystemBooster::version()));
        ui.add_space(10.0);

        // Process count
        let process_count = self.booster.get_roblox_process_count();
        ui.label(format!("📊 Roblox Processes: {process_count}"));
        ui.add_space(5.0);

        // Current Roblox path
        let current_path = self.booster.get_roblox_path();
        ui.label(format!("📂 Path: {}", 
            if current_path.len() > 40 {
                format!("...{}", &current_path[current_path.len()-37..])
            } else {
                current_path
            }
        ));
        ui.add_space(10.0);

        ui.separator();

        // Main toggle button
        let button_text = if self.is_enabled {
            "🟢 Booster: ON"
        } else {
            "🔴 Booster: OFF"
        };

        let button = egui::Button::new(button_text)
            .min_size(egui::vec2(200.0, 40.0));

        if ui.add(button).clicked() {
            self.toggle_booster();
        }

        ui.add_space(10.0);

        // Launch Roblox button
        let launch_button = egui::Button::new("🎮 Launch Roblox")
            .min_size(egui::vec2(200.0, 35.0));

        if ui.add(launch_button).clicked() {
            match self.booster.launch_roblox() {
                Ok(()) => {
                    self.status_message = "✓ Launching Roblox...".to_string();
                }
                Err(e) => {
                    self.status_message = format!("✗ Launch failed: {e}");
                }
            }
        }

        ui.add_space(10.0);

        // Settings button
        let settings_button = egui::Button::new("⚙️ Settings")
            .min_size(egui::vec2(200.0, 30.0));

        if ui.add(settings_button).clicked() {
            self.show_settings = !self.show_settings;
            self.temp_config = self.config.clone();
            self.temp_custom_path = self.config.custom_roblox_path.clone().unwrap_or_default();
        }

        ui.separator();

        // Status message
        ui.label(format!("Status: {}", self.status_message));

        // Stats display (if enabled)
        if self.is_enabled {
            let stats = self.booster.get_stats();
            ui.add_space(5.0);
            ui.label(format!("• Processes boosted: {}", stats.processes_boosted));
            ui.label(format!("• Priority: {}", self.config.priority_name()));
            ui.label(format!("• GPU boost: {}", if stats.gpu_boosted { "✓" } else { "✗" }));
            ui.label(format!("• Memory: {}", if stats.memory_cleared { "✓" } else { "✗" }));
        }
    }

    fn render_settings_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙️ Settings");
        ui.add_space(10.0);

        // Auto-start booster
        ui.checkbox(
            &mut self.temp_config.auto_start_booster,
            "🚀 Auto-start booster on launch",
        );
        ui.add_space(5.0);

        // Auto-detect Roblox
        ui.checkbox(
            &mut self.temp_config.auto_detect_roblox,
            "🔍 Auto-detect and boost Roblox",
        );
        ui.add_space(10.0);

        ui.separator();

        // CPU Priority slider
        ui.label("💻 CPU Priority Level:");
        ui.add(
            egui::Slider::new(&mut self.temp_config.priority_level, 0..=2)
                .text("Level")
                .show_value(false),
        );
        ui.label(format!("  → {}", self.temp_config.priority_name()));
        ui.add_space(10.0);

        // GPU boost toggle (v2.0 NEW)
        ui.checkbox(
            &mut self.temp_config.enable_gpu_boost,
            "🎮 Enable GPU Priority Boost (v2.0)",
        );
        ui.add_space(5.0);

        // Memory optimization
        ui.checkbox(
            &mut self.temp_config.clear_memory_cache,
            "🧹 Clear memory cache",
        );
        ui.add_space(10.0);

        ui.separator();

        // Custom Roblox Path (v2.0 NEW)
        ui.label("📂 Custom Roblox Path:");
        ui.add_space(5.0);
        
        ui.horizontal(|ui| {
            ui.label("Path:");
            let path_input = egui::TextEdit::singleline(&mut self.temp_custom_path)
                .desired_width(250.0)
                .hint_text("Leave empty for auto-detect");
            ui.add(path_input);
        });

        ui.add_space(5.0);
        ui.label("💡 Example: C:\\Users\\YourName\\AppData\\Local\\Roblox\\Versions");
        
        // Show current effective path
        let effective_path = if self.temp_custom_path.is_empty() {
            "Auto-detect (default)".to_string()
        } else {
            self.temp_custom_path.clone()
        };
        ui.label(format!("Current: {}", effective_path));
        
        ui.add_space(10.0);

        ui.separator();

        // Save/Cancel buttons
        ui.horizontal(|ui| {
            let save_button = egui::Button::new("💾 Save Settings")
                .min_size(egui::vec2(120.0, 30.0));

            if ui.add(save_button).clicked() {
                // Update custom path
                self.temp_config.custom_roblox_path = if self.temp_custom_path.is_empty() {
                    None
                } else {
                    Some(self.temp_custom_path.clone())
                };

                // Save config
                self.config = self.temp_config.clone();
                if let Err(e) = self.config.save() {
                    self.status_message = format!("✗ Save failed: {e}");
                } else {
                    self.booster.update_config(self.config.clone());
                    self.status_message = "✓ Settings saved!".to_string();
                    self.show_settings = false;
                }
            }

            ui.add_space(10.0);

            let cancel_button = egui::Button::new("❌ Cancel").min_size(egui::vec2(100.0, 30.0));

            if ui.add(cancel_button).clicked() {
                self.temp_config = self.config.clone();
                self.temp_custom_path = self.config.custom_roblox_path.clone().unwrap_or_default();
                self.show_settings = false;
            }
        });

        ui.add_space(10.0);

        // Info
        ui.label("ℹ️ Changes take effect immediately after saving.");
    }

    fn toggle_booster(&mut self) {
        if self.is_enabled {
            // Disable
            match self.booster.disable() {
                Ok(msg) => self.status_message = msg,
                Err(e) => self.status_message = format!("✗ Error: {e}"),
            }
            self.is_enabled = false;
        } else {
            // Enable
            match self.booster.enable() {
                Ok(msg) => {
                    self.status_message = msg;
                    self.is_enabled = true;
                }
                Err(e) => {
                    self.status_message = format!("✗ Error: {e}");
                    self.is_enabled = false;
                }
            }
        }
    }

    fn check_auto_boost(&mut self) {
        if !self.is_enabled {
            return;
        }

        if self.last_auto_check.elapsed() >= Duration::from_secs(2) {
            if let Some(msg) = self.booster.auto_detect_and_boost() {
                self.status_message = msg;
            }
            self.last_auto_check = Instant::now();
        }
    }
}

impl eframe::App for RobloxBoosterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-boost check
        self.check_auto_boost();

        // Request repaint for auto-detect
        ctx.request_repaint_after(Duration::from_secs(2));

        // Main panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if self.show_settings {
                    self.render_settings_ui(ui);
                } else {
                    self.render_main_ui(ui);
                }
            });
        });
    }
}