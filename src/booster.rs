use crate::config::OptimizationLevel;
use anyhow::Result;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::{
    OpenProcess, SetPriorityClass, ABOVE_NORMAL_PRIORITY_CLASS, HIGH_PRIORITY_CLASS,
    NORMAL_PRIORITY_CLASS, PROCESS_SET_INFORMATION,
};

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{CloseHandle, HANDLE};

#[cfg(target_os = "windows")]
use windows::Win32::System::ProcessStatus::K32EmptyWorkingSet;

pub struct Booster {
    system: System,
}

impl Booster {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    /// Tìm process Roblox
    pub fn find_roblox_pid(&mut self) -> Option<u32> {
        self.system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            ProcessRefreshKind::default(),
        );

        self.system
            .processes()
            .values()
            .find(|p| {
                p.name().to_lowercase().contains("roblox")
                    && !p.name().to_lowercase().contains("studio")
            })
            .map(|p| p.pid().as_u32())
    }

    /// Boost Roblox với optimization level
    #[cfg(target_os = "windows")]
    pub fn boost_roblox(&mut self, level: OptimizationLevel) -> Result<String> {
        let Some(pid) = self.find_roblox_pid() else {
            anyhow::bail!("Không tìm thấy Roblox đang chạy");
        };

        unsafe {
            let handle = OpenProcess(PROCESS_SET_INFORMATION, false, pid)?;

            // Set CPU priority
            let priority = match level {
                OptimizationLevel::Low => NORMAL_PRIORITY_CLASS,
                OptimizationLevel::Medium => ABOVE_NORMAL_PRIORITY_CLASS,
                OptimizationLevel::High => HIGH_PRIORITY_CLASS,
            };

            SetPriorityClass(handle, priority)?;

            // Optimize memory (trim working set)
            if matches!(level, OptimizationLevel::Medium | OptimizationLevel::High) {
                K32EmptyWorkingSet(handle)?;
            }

            CloseHandle(handle).ok();
        }

        Ok(format!("Đã boost Roblox (PID: {pid}) - Level: {level:?}"))
    }

    #[cfg(not(target_os = "windows"))]
    pub fn boost_roblox(&mut self, _level: OptimizationLevel) -> Result<String> {
        anyhow::bail!("Chỉ hỗ trợ Windows")
    }

    /// Kiểm tra Roblox có đang chạy không
    pub fn is_roblox_running(&mut self) -> bool {
        self.find_roblox_pid().is_some()
    }
}