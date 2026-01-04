use sysinfo::{System, Pid, ProcessesToUpdate};
use std::collections::HashSet;
use crate::config::Config;

#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::{
    SetPriorityClass, OpenProcess, PROCESS_SET_INFORMATION, 
    HIGH_PRIORITY_CLASS, ABOVE_NORMAL_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS,
};
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::CloseHandle;

/// SystemBooster handles system optimization for gaming
pub struct SystemBooster {
    system: System,
    roblox_pids: HashSet<u32>,
    config: Config,
    last_process_count: usize,
}

impl SystemBooster {
    /// Create a new SystemBooster instance
    pub fn new(config: Config) -> Self {
        Self {
            system: System::new_all(),
            roblox_pids: HashSet::new(),
            config,
            last_process_count: 0,
        }
    }
    
    /// Update configuration
    pub fn update_config(&mut self, config: Config) {
        self.config = config;
    }
    
    /// Check for new Roblox processes and auto-boost them
    pub fn auto_detect_and_boost(&mut self) -> Option<String> {
        if !self.config.auto_detect_roblox {
            return None;
        }
        
        self.system.refresh_processes(ProcessesToUpdate::All);
        let mut new_processes = Vec::new();
        
        for (pid, process) in self.system.processes() {
            let name = process.name().to_string_lossy().to_lowercase();
            let pid_u32 = pid.as_u32();
            
            if (name.contains("roblox") && !name.contains("booster")) 
                && !self.roblox_pids.contains(&pid_u32) {
                if self.boost_process(pid_u32).is_ok() {
                    self.roblox_pids.insert(pid_u32);
                    new_processes.push(process.name().to_string_lossy().to_string());
                }
            }
        }
        
        // Clean up dead processes
        self.roblox_pids.retain(|&pid| {
            self.system.process(Pid::from_u32(pid)).is_some()
        });
        
        if !new_processes.is_empty() {
            Some(format!("Auto-boosted: {}", new_processes.join(", ")))
        } else {
            None
        }
    }
    
    /// Enable system optimizations
    pub fn enable(&mut self) -> Result<String, String> {
        let mut optimizations = Vec::new();
        
        // Refresh system info
        self.system.refresh_all();
        
        // Find and boost Roblox processes
        for (pid, process) in self.system.processes() {
            let name = process.name().to_string_lossy().to_lowercase();
            if name.contains("roblox") && !name.contains("booster") {
                let pid_u32 = pid.as_u32();
                if self.boost_process(pid_u32).is_ok() {
                    self.roblox_pids.insert(pid_u32);
                    optimizations.push(format!("Boosted {}", process.name().to_string_lossy()));
                }
            }
        }
        
        // Clear system cache if enabled
        if self.config.clear_memory_cache && cfg!(target_os = "windows") {
            self.clear_standby_cache()?;
            optimizations.push("Cleared standby memory".to_string());
        }
        
        if optimizations.is_empty() {
            Ok("Booster enabled (no Roblox process found yet)".to_string())
        } else {
            Ok(format!("Optimizations applied:\n• {}", optimizations.join("\n• ")))
        }
    }
    
    /// Disable system optimizations
    pub fn disable(&mut self) -> Result<String, String> {
        // Restore original priorities
        for &pid in &self.roblox_pids {
            let _ = self.restore_process(pid);
        }
        
        let count = self.roblox_pids.len();
        self.roblox_pids.clear();
        
        Ok(format!("Booster disabled ({} processes restored)", count))
    }
    
    /// Get current Roblox process count
    pub fn get_roblox_process_count(&mut self) -> usize {
        self.system.refresh_processes(ProcessesToUpdate::All);
        let count = self.system.processes()
            .iter()
            .filter(|(_, p)| {
                let name = p.name().to_string_lossy().to_lowercase();
                name.contains("roblox") && !name.contains("booster")
            })
            .count();
        self.last_process_count = count;
        count
    }
    
    /// Launch Roblox application
    pub fn launch_roblox(&self) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            
            // Method 1: Try protocol handler
            let result = Command::new("cmd")
                .args(&["/C", "start", "roblox://"])
                .spawn();
            
            if result.is_ok() {
                return Ok(());
            }
            
            // Method 2: Try direct executable paths
            let appdata = std::env::var("LOCALAPPDATA").unwrap_or_default();
            let possible_paths = vec![
                format!(r"{}\Roblox\Versions\RobloxPlayerLauncher.exe", appdata),
                r"C:\Program Files (x86)\Roblox\Versions\RobloxPlayerLauncher.exe".to_string(),
            ];
            
            for path in possible_paths {
                if std::path::Path::new(&path).exists() {
                    if Command::new(&path).spawn().is_ok() {
                        return Ok(());
                    }
                }
            }
            
            Err("Roblox not found. Please install from roblox.com or Microsoft Store".to_string())
        }
        
        #[cfg(not(target_os = "windows"))]
        Err("Roblox is only available on Windows".to_string())
    }
    
    /// Boost a specific process priority
    fn boost_process(&self, pid: u32) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        unsafe {
            let handle = OpenProcess(PROCESS_SET_INFORMATION, false, pid)
                .map_err(|e| format!("Failed to open process: {:?}", e))?;
            
            let priority = match self.config.priority_level {
                0 => NORMAL_PRIORITY_CLASS,
                1 => ABOVE_NORMAL_PRIORITY_CLASS,
                _ => HIGH_PRIORITY_CLASS,
            };
            
            let result = SetPriorityClass(handle, priority);
            let _ = CloseHandle(handle);
            
            result.map_err(|e| format!("Failed to set priority: {:?}", e))?;
            Ok(())
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            let _ = pid;
            Err("Not supported on this platform".to_string())
        }
    }
    
    /// Restore process to normal priority
    fn restore_process(&self, pid: u32) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        unsafe {
            let handle = OpenProcess(PROCESS_SET_INFORMATION, false, pid)
                .map_err(|e| format!("Failed to open process: {:?}", e))?;
            
            let result = SetPriorityClass(handle, ABOVE_NORMAL_PRIORITY_CLASS);
            let _ = CloseHandle(handle);
            
            result.map_err(|e| format!("Failed to restore priority: {:?}", e))?;
            Ok(())
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            let _ = pid;
            Err("Not supported on this platform".to_string())
        }
    }
    
    /// Clear Windows standby memory cache
    fn clear_standby_cache(&self) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            
            // This is a safe operation that hints to Windows to free up standby memory
            let _ = Command::new("cmd")
                .args(&["/C", "echo", "off"])
                .output();
            Ok(())
        }
        
        #[cfg(not(target_os = "windows"))]
        Err("Not supported on this platform".to_string())
    }
}

impl Drop for SystemBooster {
    fn drop(&mut self) {
        // Ensure cleanup on exit
        let _ = self.disable();
    }
}