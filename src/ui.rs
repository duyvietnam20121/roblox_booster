use crate::booster::RobloxBoosterEngine;
use crate::config::Config;
use iced::{
    widget::{button, checkbox, column, container, row, text, Column},
    Alignment, Application, Command, Element, Length, Theme,
};
use std::sync::Arc;

/// Message types cho UI
#[derive(Debug, Clone)]
pub enum Message {
    ToggleBooster,
    OpenSettings,
    CloseSettings,
    
    // Config toggles
    ToggleAutoStart(bool),
    ToggleTimerResolution(bool),
    ToggleMemoryCleanup(bool),
    ToggleAutoDetection(bool),
    
    SaveSettings,
}

/// Main application struct
pub struct RobloxBooster {
    booster: Arc<RobloxBoosterEngine>,
    is_boosting: bool,
    show_settings: bool,
    config: Config,
}

impl Application for RobloxBooster {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let config = Config::load();
        
        (
            Self {
                booster: Arc::new(RobloxBoosterEngine::new(config.clone())),
                is_boosting: false,
                show_settings: false,
                config,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Roblox Booster - No Admin Required")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleBooster => {
                if self.is_boosting {
                    self.booster.stop();
                    self.is_boosting = false;
                } else {
                    // Recreate booster với config hiện tại
                    self.booster = Arc::new(RobloxBoosterEngine::new(self.config.clone()));
                    
                    let booster = Arc::clone(&self.booster);
                    tokio::spawn(async move {
                        booster.start().await;
                    });
                    self.is_boosting = true;
                }
            }
            Message::OpenSettings => {
                self.show_settings = true;
            }
            Message::CloseSettings => {
                self.show_settings = false;
            }
            
            // Config updates
            Message::ToggleAutoStart(value) => {
                self.config.auto_start = value;
            }
            Message::ToggleTimerResolution(value) => {
                self.config.enable_timer_resolution = value;
            }
            Message::ToggleMemoryCleanup(value) => {
                self.config.enable_memory_cleanup = value;
            }
            Message::ToggleAutoDetection(value) => {
                self.config.enable_auto_detection = value;
            }
            
            Message::SaveSettings => {
                let _ = self.config.save();
                self.show_settings = false;
                
                // Nếu đang chạy, restart với config mới
                if self.is_boosting {
                    self.booster.stop();
                    self.booster = Arc::new(RobloxBoosterEngine::new(self.config.clone()));
                    
                    let booster = Arc::clone(&self.booster);
                    tokio::spawn(async move {
                        booster.start().await;
                    });
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        if self.show_settings {
            self.settings_view()
        } else {
            self.main_view()
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl RobloxBooster {
    /// View chính
    fn main_view(&self) -> Element<Message> {
        let status_text = if self.is_boosting {
            "🚀 AUTO BOOSTER: ĐANG CHẠY"
        } else {
            "⏸️ AUTO BOOSTER: TẮT"
        };

        let toggle_button = button(
            text(if self.is_boosting {
                "TẮT AUTO BOOSTER"
            } else {
                "BẬT AUTO BOOSTER"
            })
            .size(20),
        )
        .padding(20)
        .on_press(Message::ToggleBooster);

        let settings_button = button(text("⚙️ SETTINGS").size(16))
            .padding(15)
            .on_press(Message::OpenSettings);
        
        // Active features
        let features_text = self.get_features_summary();

        let content = column![
            text("ROBLOX BOOSTER").size(28),
            text("(Không cần Admin)").size(14),
            text("").size(5),
            text(status_text).size(18),
            text("").size(10),
            text(features_text).size(12),
            text("").size(15),
            toggle_button,
            settings_button,
            text("").size(10),
            text("💡 Tip: Mở Roblox và app sẽ tự tối ưu").size(11),
        ]
        .spacing(12)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    /// View settings
    fn settings_view(&self) -> Element<Message> {
        let content: Column<Message> = column![
            text("⚙️ CÀI ĐẶT").size(26),
            text("").size(5),
            
            // General
            text("Chung:").size(18),
            checkbox("Auto Start khi mở app", self.config.auto_start)
                .on_toggle(Message::ToggleAutoStart),
            
            text("").size(10),
            
            // Features (KHÔNG CẦN ADMIN)
            text("Tính Năng:").size(18),
            
            checkbox(
                "⏱️  Timer Resolution (1ms)\n    → Giảm lag, game mượt hơn",
                self.config.enable_timer_resolution
            )
            .on_toggle(Message::ToggleTimerResolution),
            
            checkbox(
                "🧹 Memory Cleanup (60s)\n    → Giải phóng RAM cho Roblox",
                self.config.enable_memory_cleanup
            )
            .on_toggle(Message::ToggleMemoryCleanup),
            
            checkbox(
                "🔍 Auto-Detection\n    → Tự động phát hiện Roblox",
                self.config.enable_auto_detection
            )
            .on_toggle(Message::ToggleAutoDetection),
            
            text("").size(15),
            
            // Info
            text("ℹ️  Tất cả features KHÔNG cần Admin").size(11),
            text("ℹ️  Tối ưu system-wide cho mọi app").size(11),
            
            text("").size(10),
            
            // Buttons
            row![
                button(text("💾 LƯU").size(16))
                    .padding(15)
                    .on_press(Message::SaveSettings),
                button(text("❌ HỦY").size(16))
                    .padding(15)
                    .on_press(Message::CloseSettings),
            ]
            .spacing(10),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
    
    /// Summary của active features
    fn get_features_summary(&self) -> String {
        let mut active = Vec::new();
        
        if self.config.enable_timer_resolution {
            active.push("⏱️ Timer 1ms");
        }
        if self.config.enable_memory_cleanup {
            active.push("🧹 RAM Cleanup");
        }
        if self.config.enable_auto_detection {
            active.push("🔍 Auto-Detect");
        }
        
        if active.is_empty() {
            "❌ Không có feature nào được bật".to_string()
        } else {
            format!("Features: {}", active.join(" | "))
        }
    }
}