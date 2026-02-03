use sysinfo::System;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;
use crate::config::Config;

#[cfg(target_os = "windows")]
use windows::{
    Win32::System::ProcessStatus::EmptyWorkingSet,
    Win32::System::Threading::GetCurrentProcess,
};

/// Struct chính quản lý việc boost - KHÔNG CẦN ADMIN
pub struct RobloxBoosterEngine {
    is_running: Arc<Mutex<bool>>,
    sys: Arc<Mutex<System>>,
    config: Arc<Mutex<Config>>,
    timer_resolution_active: Arc<Mutex<bool>>,
}

impl RobloxBoosterEngine {
    pub fn new(config: Config) -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            sys: Arc::new(Mutex::new(System::new_all())),
            config: Arc::new(Mutex::new(config)),
            timer_resolution_active: Arc::new(Mutex::new(false)),
        }
    }

    /// Bắt đầu auto boost - KHÔNG CẦN ADMIN
    pub async fn start(&self) {
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = true;
        drop(is_running);

        println!("\n╔══════════════════════════════════════╗");
        println!("║  ĐANG KHỞI ĐỘNG BOOSTER (No Admin)  ║");
        println!("╚══════════════════════════════════════╝\n");

        // Áp dụng Timer Resolution (system-wide, không cần admin)
        self.apply_timer_resolution();

        let is_running_clone = Arc::clone(&self.is_running);
        let sys_clone = Arc::clone(&self.sys);
        let config_clone = Arc::clone(&self.config);

        tokio::spawn(async move {
            let interval_seconds = {
                let config = config_clone.lock().unwrap().clone();
                if config.boost_interval_seconds == 0 {
                    60
                } else {
                    config.boost_interval_seconds
                }
            };
            let mut interval = time::interval(Duration::from_secs(interval_seconds));

            loop {
                interval.tick().await;

                let running = *is_running_clone.lock().unwrap();
                if !running {
                    break;
                }

                // Refresh system info
                let mut sys = sys_clone.lock().unwrap();
                sys.refresh_all();

                let config = config_clone.lock().unwrap().clone();

                // Detect Roblox (chỉ để hiển thị status)
                if config.enable_auto_detection {
                    Self::detect_roblox(&sys);
                }
                
                // Memory cleanup (dọn RAM của app này)
                if config.enable_memory_cleanup {
                    Self::cleanup_memory();
                }

                println!("⏱️  Cycle hoàn tất (next: {}s)\n", interval_seconds);
            }
        });
        
        println!("🚀 Auto Booster đã BẬT");
        let interval_seconds = self.config.lock().unwrap().boost_interval_seconds;
        let interval_seconds = if interval_seconds == 0 { 60 } else { interval_seconds };
        println!("⏱️  Boost interval: {} giây", interval_seconds);
        println!("ℹ️  Chế độ: Không cần Admin\n");
    }

    /// Dừng auto boost
    pub fn stop(&self) {
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
        
        // Restore timer resolution
        self.restore_timer_resolution();
        
        println!("\n⏸️  Auto Booster đã TẮT\n");
    }

    /// Kiểm tra trạng thái
    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }

    // ========================================
    // TÍNH NĂNG 1: TIMER RESOLUTION (System-wide)
    // ========================================
    
    /// Set timer resolution to 1ms - KHÔNG CẦN ADMIN
    /// Áp dụng cho toàn hệ thống, benefit cho tất cả apps
    fn apply_timer_resolution(&self) {
        let config = self.config.lock().unwrap();
        
        if !config.enable_timer_resolution {
            println!("⏱️  Timer Resolution: TẮT");
            return;
        }
        
        #[cfg(target_os = "windows")]
        {
            use std::ffi::CString;
            
            unsafe {
                let winmm = match windows::Win32::System::LibraryLoader::LoadLibraryA(
                    windows::core::PCSTR("winmm.dll\0".as_ptr())
                ) {
                    Ok(lib) => lib,
                    Err(_) => {
                        eprintln!("✗ Không load được winmm.dll");
                        return;
                    }
                };
                
                let func_name = CString::new("timeBeginPeriod").unwrap();
                let time_begin_period = windows::Win32::System::LibraryLoader::GetProcAddress(
                    winmm,
                    windows::core::PCSTR(func_name.as_ptr() as *const u8)
                );
                
                if let Some(func) = time_begin_period {
                    let time_begin: extern "system" fn(u32) -> u32 = std::mem::transmute(func);
                    let result = time_begin(1); // 1ms resolution
                    
                    if result == 0 {
                        println!("✓ Timer Resolution: 1ms (system-wide)");
                        println!("  ℹ️  Benefit: Mọi app đều mượt hơn");
                        let mut active = self.timer_resolution_active.lock().unwrap();
                        *active = true;
                    } else {
                        eprintln!("✗ Không set được timer resolution (error: {})", result);
                    }
                }
            }
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            println!("⚠️  Timer Resolution chỉ hỗ trợ Windows");
        }
    }
    
    /// Restore timer resolution - KHÔNG CẦN ADMIN
    fn restore_timer_resolution(&self) {
        let active = *self.timer_resolution_active.lock().unwrap();
        
        if !active {
            return;
        }
        
        #[cfg(target_os = "windows")]
        {
            use std::ffi::CString;
            
            unsafe {
                if let Ok(winmm) = windows::Win32::System::LibraryLoader::LoadLibraryA(
                    windows::core::PCSTR("winmm.dll\0".as_ptr())
                ) {
                    let func_name = CString::new("timeEndPeriod").unwrap();
                    if let Some(func) = windows::Win32::System::LibraryLoader::GetProcAddress(
                        winmm,
                        windows::core::PCSTR(func_name.as_ptr() as *const u8)
                    ) {
                        let time_end: extern "system" fn(u32) -> u32 = std::mem::transmute(func);
                        time_end(1);
                        println!("✓ Timer Resolution đã restore");
                    }
                }
            }
        }
        
        let mut active = self.timer_resolution_active.lock().unwrap();
        *active = false;
    }

    // ========================================
    // TÍNH NĂNG 2: ROBLOX DETECTION (Chỉ hiển thị)
    // ========================================
    
    /// Phát hiện Roblox để hiển thị status - KHÔNG CẦN ADMIN
    fn detect_roblox(sys: &System) {
        let mut found_processes = Vec::new();
        
        for (pid, process) in sys.processes() {
            let name = process.name().to_lowercase();
            
            if name.contains("roblox") || name.contains("robloxplayerbeta") {
                found_processes.push((process.name().to_string(), pid.as_u32()));
            }
        }
        
        if found_processes.is_empty() {
            println!("🔍 Status: Không phát hiện Roblox");
        } else {
            println!("🎮 Phát hiện Roblox:");
            for (name, pid) in found_processes {
                println!("   • {} (PID: {})", name, pid);
            }
            println!("   ℹ️  System đang được tối ưu cho gaming");
        }
    }

    // ========================================
    // TÍNH NĂNG 3: MEMORY CLEANUP (Current process)
    // ========================================
    
    /// Dọn RAM của chính app này - KHÔNG CẦN ADMIN
    /// Giải phóng RAM cho Roblox sử dụng
    #[cfg(target_os = "windows")]
    fn cleanup_memory() {
        unsafe {
            match GetCurrentProcess() {
                Ok(handle) => {
                    match EmptyWorkingSet(handle) {
                        Ok(_) => {
                            println!("✓ Memory Cleanup: Đã giải phóng RAM");
                            println!("  ℹ️  RAM available tăng lên cho Roblox");
                        }
                        Err(e) => {
                            eprintln!("✗ Memory cleanup thất bại: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("✗ GetCurrentProcess thất bại: {:?}", e);
                }
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    fn cleanup_memory() {
        println!("⚠️  Memory cleanup chỉ hỗ trợ Windows");
    }
}

impl Drop for RobloxBoosterEngine {
    fn drop(&mut self) {
        // Ensure timer resolution được restore
        self.restore_timer_resolution();
    }
}
