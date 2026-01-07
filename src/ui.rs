use crate::{booster::SystemBooster, config::Config};
use eframe::egui;
use std::time::{Duration, Instant};

const AUTO_DETECT_INTERVAL: Duration = Duration::from_secs(2);
const BUTTON_SIZE: egui::Vec2 = egui::vec2(220.0, 50.0);
const SETTINGS_BUTTON_SIZE: egui::Vec2 = egui::vec2(220.0, 35.0);

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
    #[must_use]
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = Config::load();
        let auto_start = config.auto_start_booster;
        let mut booster = SystemBooster::new(config.clone());
        
        let (status_message, booster_enabled) = if auto_start {
            match booster.enable() {
                Ok(msg) => (msg, true),
                Err(e) => (format!("Auto-start failed: {e}"), false),
            }
        } else {
            (String::from("Ready - Click to enable booster"), false)
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
    
    fn render_main_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("🚀 Roblox Booster");
        ui.add_space(10.0);
        
        self.render_status(ui);
        
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(15.0);
        
        self.render_controls(ui);
        
        ui.add_space(15.0);
        ui.separator();
        ui.add_space(8.0);
        
        self.render_info(ui);
    }
    
    fn render_status(&self, ui: &mut egui::Ui) {
        // Main status
        ui.horizontal(|ui| {
            ui.label("Status:");
            let color = if self.booster_enabled {
                egui::Color32::GREEN
            } else {
                egui::Color32::GRAY
            };
            ui.colored_label(
                color,
                if self.booster_enabled {
                    "ACTIVE"
                } else {
                    "INACTIVE"
                },
            );
        });
        
        // Process count
        if self.booster_enabled {
            ui.horizontal(|ui| {
                ui.label("Roblox processes:");
                ui.colored_label(egui::Color32::LIGHT_BLUE, self.roblox_count.to_string());
            });
            
            // Optimization stats
            let stats = self.booster.get_stats();
            if stats.processes_boosted > 0 {
                ui.horizontal(|ui| {
                    ui.label("Optimized:");
                    ui.colored_label(
                        egui::Color32::GREEN,
                        format!("{} process(es)", stats.processes_boosted),
                    );
                });
                
                ui.horizontal(|ui| {
                    ui.label("Priority:");
                    let priority_text = match stats.priority_level {
                        0 => "Normal",
                        1 => "Above Normal",
                        _ => "High",
                    };
                    ui.colored_label(egui::Color32::YELLOW, priority_text);
                });
            }
        }
        
        ui.add_space(5.0);
        
        // Status message in a scrollable area
        egui::ScrollArea::vertical()
            .max_height(100.0)
            .show(ui, |ui| {
                ui.label(&self.status_message);
            });
    }
    
    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            let button_text = if self.booster_enabled {
                "🟢 BOOSTER: ON"
            } else {
                "⚪ BOOSTER: OFF"
            };
            
            let button_color = if self.booster_enabled {
                egui::Color32::from_rgb(46, 204, 113)
            } else {
                egui::Color32::from_rgb(149, 165, 166)
            };
            
            let button = egui::Button::new(
                egui::RichText::new(button_text)
                    .size(16.0)
                    .color(egui::Color32::WHITE),
            )
            .fill(button_color)
            .min_size(BUTTON_SIZE);
            
            if ui.add(button).clicked() {
                self.toggle_booster();
            }
            
            ui.add_space(12.0);
            
            let launch_button =
                egui::Button::new(egui::RichText::new("🎮 Launch Roblox").size(15.0))
                    .min_size(BUTTON_SIZE);
            
            if ui.add(launch_button).clicked() {
                self.launch_roblox();
            }
            
            ui.add_space(12.0);
            
            if ui
                .add(egui::Button::new("⚙️ Settings").min_size(SETTINGS_BUTTON_SIZE))
                .clicked()
            {
                self.show_settings = !self.show_settings;
            }
        });
    }
    
    fn render_info(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("Optimizes CPU priority & memory")
                    .size(11.0)
                    .color(egui::Color32::GRAY),
            );
            ui.label(
                egui::RichText::new("for better gaming performance")
                    .size(11.0)
                    .color(egui::Color32::GRAY),
            );
        });
    }
    
    fn render_settings_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("⚙️ Settings")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                let mut config_changed = false;
                
                ui.heading("Booster Settings");
                ui.add_space(10.0);
                
                config_changed |= ui
                    .checkbox(
                        &mut self.config.auto_start_booster,
                        "Auto-start booster on launch",
                    )
                    .on_hover_text("Automatically enable booster when app starts")
                    .changed();
                
                ui.add_space(5.0);
                
                config_changed |= ui
                    .checkbox(&mut self.config.auto_detect_roblox, "Auto-detect Roblox")
                    .on_hover_text("Automatically optimize Roblox when detected")
                    .changed();
                
                ui.add_space(5.0);
                
                config_changed |= ui
                    .checkbox(&mut self.config.clear_memory_cache, "Optimize memory")
                    .on_hover_text("Trim working set and optimize system memory")
                    .changed();
                
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                
                ui.label(egui::RichText::new("Process Priority Level:").strong());
                ui.add_space(5.0);
                
                let mut priority = self.config.priority_level as usize;
                
                ui.vertical(|ui| {
                    config_changed |= ui
                        .radio_value(&mut priority, 0, "Normal")
                        .on_hover_text("Standard priority (safest)")
                        .changed();
                    
                    config_changed |= ui
                        .radio_value(&mut priority, 1, "Above Normal")
                        .on_hover_text("Higher priority (recommended)")
                        .changed();
                    
                    config_changed |= ui
                        .radio_value(&mut priority, 2, "High")
                        .on_hover_text("Highest priority (may affect system)")
                        .changed();
                });
                
                self.config.priority_level = priority as u8;
                
                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    if ui.button("💾 Save").clicked() {
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
                    
                    if ui.button("❌ Cancel").clicked() {
                        self.config = Config::load();
                        self.show_settings = false;
                    }
                    
                    if config_changed {
                        ui.colored_label(egui::Color32::YELLOW, "⚠ Unsaved");
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
        
        ctx.request_repaint_after(AUTO_DETECT_INTERVAL);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_ui(ui);
        });
        
        if self.show_settings {
            self.render_settings_window(ctx);
        }
    }
}