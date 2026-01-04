use crate::booster::SystemBooster;
use crate::config::Config;
use eframe::egui;
use std::time::{Duration, Instant};

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
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = Config::load();
        let auto_start = config.auto_start_booster;
        let mut booster = SystemBooster::new(config.clone());
        
        let mut status_message = String::from("Ready");
        let mut booster_enabled = false;
        
        // Auto-start booster if configured
        if auto_start {
            match booster.enable() {
                Ok(msg) => {
                    status_message = msg;
                    booster_enabled = true;
                }
                Err(e) => {
                    status_message = format!("Auto-start failed: {}", e);
                }
            }
        }
        
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
}

impl eframe::App for RobloxBoosterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-detect every 2 seconds
        if self.booster_enabled && self.last_check.elapsed() > Duration::from_secs(2) {
            self.last_check = Instant::now();
            
            if let Some(msg) = self.booster.auto_detect_and_boost() {
                self.status_message = msg;
            }
            
            self.roblox_count = self.booster.get_roblox_process_count();
        }
        
        // Request repaint for smooth UI updates
        ctx.request_repaint_after(Duration::from_secs(2));
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🚀 Roblox Booster");
            ui.add_space(10.0);
            
            // Status display
            ui.horizontal(|ui| {
                ui.label("Status:");
                let color = if self.booster_enabled {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::GRAY
                };
                ui.colored_label(color, &self.status_message);
            });
            
            // Roblox process count
            if self.booster_enabled {
                ui.horizontal(|ui| {
                    ui.label("Roblox processes:");
                    ui.colored_label(egui::Color32::LIGHT_BLUE, format!("{}", self.roblox_count));
                });
            }
            
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(15.0);
            
            // Main controls
            ui.vertical_centered(|ui| {
                // Auto Booster Toggle
                let button_text = if self.booster_enabled {
                    "🟢 Booster: ON"
                } else {
                    "⚪ Booster: OFF"
                };
                
                let button = egui::Button::new(button_text)
                    .min_size(egui::vec2(220.0, 50.0));
                
                if ui.add(button).clicked() {
                    self.booster_enabled = !self.booster_enabled;
                    
                    if self.booster_enabled {
                        match self.booster.enable() {
                            Ok(msg) => {
                                self.status_message = msg;
                                self.roblox_count = self.booster.get_roblox_process_count();
                            }
                            Err(e) => {
                                self.status_message = format!("Error: {}", e);
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
                                self.status_message = format!("Error: {}", e);
                            }
                        }
                    }
                }
                
                ui.add_space(12.0);
                
                // Launch Roblox Button
                let launch_button = egui::Button::new("🎮 Launch Roblox")
                    .min_size(egui::vec2(220.0, 50.0));
                
                if ui.add(launch_button).clicked() {
                    match self.booster.launch_roblox() {
                        Ok(_) => {
                            self.status_message = String::from("Launching Roblox...");
                        }
                        Err(e) => {
                            self.status_message = format!("Launch failed: {}", e);
                        }
                    }
                }
                
                ui.add_space(12.0);
                
                // Settings Button
                let settings_button = egui::Button::new("⚙️ Settings")
                    .min_size(egui::vec2(220.0, 35.0));
                
                if ui.add(settings_button).clicked() {
                    self.show_settings = !self.show_settings;
                }
            });
            
            ui.add_space(15.0);
            ui.separator();
            ui.add_space(8.0);
            
            // Info section
            ui.vertical_centered(|ui| {
                ui.label("Booster optimizes system resources");
                ui.label("for better gaming performance");
            });
        });
        
        // Settings window
        if self.show_settings {
            egui::Window::new("⚙️ Settings")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    let mut config_changed = false;
                    
                    ui.heading("Booster Settings");
                    ui.add_space(10.0);
                    
                    // Auto-start booster
                    if ui.checkbox(&mut self.config.auto_start_booster, "Auto-start booster on launch")
                        .changed() {
                        config_changed = true;
                    }
                    
                    ui.add_space(5.0);
                    
                    // Auto-detect
                    if ui.checkbox(&mut self.config.auto_detect_roblox, "Auto-detect and boost Roblox")
                        .on_hover_text("Automatically boost Roblox when detected")
                        .changed() {
                        config_changed = true;
                    }
                    
                    ui.add_space(5.0);
                    
                    // Clear cache
                    if ui.checkbox(&mut self.config.clear_memory_cache, "Clear memory cache when boosting")
                        .changed() {
                        config_changed = true;
                    }
                    
                    ui.add_space(10.0);
                    
                    // Priority level
                    ui.label("Process Priority Level:");
                    let mut priority = self.config.priority_level as usize;
                    ui.horizontal(|ui| {
                        if ui.radio_value(&mut priority, 0, "Normal").changed() {
                            config_changed = true;
                        }
                        if ui.radio_value(&mut priority, 1, "Above Normal").changed() {
                            config_changed = true;
                        }
                        if ui.radio_value(&mut priority, 2, "High").changed() {
                            config_changed = true;
                        }
                    });
                    self.config.priority_level = priority as u8;
                    
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    // Save/Close buttons
                    ui.horizontal(|ui| {
                        if ui.button("💾 Save").clicked() {
                            match self.config.save() {
                                Ok(_) => {
                                    self.booster.update_config(self.config.clone());
                                    self.status_message = String::from("Settings saved ✓");
                                    self.show_settings = false;
                                }
                                Err(e) => {
                                    self.status_message = format!("Save failed: {}", e);
                                }
                            }
                        }
                        
                        if ui.button("❌ Cancel").clicked() {
                            self.config = Config::load();
                            self.show_settings = false;
                        }
                        
                        if config_changed {
                            ui.colored_label(egui::Color32::YELLOW, "⚠ Unsaved changes");
                        }
                    });
                });
        }
    }
}