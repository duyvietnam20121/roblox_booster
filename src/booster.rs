use crate::config::OptimizationLevel;
use anyhow::{Context, Result};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::CloseHandle,
    System::Threading::{
        OpenProcess, SetPriorityClass, ABOVE_NORMAL_PRIORITY_CLASS, HIGH_PRIORITY_CLASS,
        NORMAL_PRIORITY_CLASS, PROCESS_SET_INFORMATION,
    },
};

pub struct Booster {
    system: System,
}

impl Default for Booster {
    fn default() -> Self {
        Self::new()
    }
}

impl Booster {
    #[must_use]
    pub fn new() -> Self {
        Self {
            system: System::new(),
        }
    }

    /// Tìm process Roblox (không bao gồm Studio)
    pub fn find_roblox_pid(&mut self) -> Option<u32> {
        // Refresh process list
        self.system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::new(),
        );

        // Tìm process có tên chứa "roblox" nhưng không phải "studio"
        self.system
            .processes()
            .values()
            .find(|p| {
                let name = p.name().to_string_lossy().to_ascii_lowercase();
                // Chỉ chấp nhận RobloxPlayerBeta.exe hoặc Roblox.exe
                // Loại trừ: Studio, Installer, Uninstaller, Crash Reporter
                name.contains("roblox")
                    && !name.contains("studio")
                    && !name.contains("install")
                    && !name.contains("crash")
            })
            .map(|p| p.pid().as_u32())
    }

    /// Lấy tên process từ PID (để verify)
    #[cfg(target_os = "windows")]
    pub fn get_process_name(&mut self, pid: u32) -> Option<String> {
        self.system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::new(),
        );

        self.system
            .processes()
            .values()
            .find(|p| p.pid().as_u32() == pid)
            .map(|p| p.name().to_string_lossy().to_string())
    }

    /// Boost Roblox - CHỈ thay đổi CPU Priority (100% an toàn)
    #[cfg(target_os = "windows")]
    pub fn boost_roblox(&mut self, level: OptimizationLevel) -> Result<String> {
        let pid = self
            .find_roblox_pid()
            .context("Không tìm thấy Roblox đang chạy")?;

        // Verify process name để đảm bảo không boost nhầm
        let process_name = self
            .get_process_name(pid)
            .context("Không thể xác minh tên process")?;

        // Chọn priority level
        let priority = match level {
            OptimizationLevel::Low => NORMAL_PRIORITY_CLASS,
            OptimizationLevel::Medium => ABOVE_NORMAL_PRIORITY_CLASS,
            OptimizationLevel::High => HIGH_PRIORITY_CLASS,
        };

        // SAFETY: Chỉ thay đổi priority, không đọc/ghi memory
        unsafe {
            // Mở process handle với quyền SET_INFORMATION
            let handle = OpenProcess(PROCESS_SET_INFORMATION, false, pid)
                .context("Không thể mở process (cần quyền admin?)")?;

            // Set CPU priority
            let result = SetPriorityClass(handle, priority);

            // CRITICAL: Luôn đóng handle
            CloseHandle(handle).ok();

            // Check kết quả
            result.context("Không thể set priority")?;
        }

        let level_name = match level {
            OptimizationLevel::Low => "Normal",
            OptimizationLevel::Medium => "Above Normal",
            OptimizationLevel::High => "High",
        };

        Ok(format!(
            "✅ Boosted {process_name} (PID: {pid}) → {level_name} Priority"
        ))
    }

    #[cfg(not(target_os = "windows"))]
    pub fn boost_roblox(&mut self, _level: OptimizationLevel) -> Result<String> {
        anyhow::bail!("Chỉ hỗ trợ Windows")
    }

    /// Kiểm tra Roblox có đang chạy không
    pub fn is_roblox_running(&mut self) -> bool {
        self.find_roblox_pid().is_some()
    }

    /// Lấy thông tin chi tiết về process Roblox
    #[cfg(target_os = "windows")]
    pub fn get_roblox_info(&mut self) -> Option<ProcessInfo> {
        let pid = self.find_roblox_pid()?;
        let name = self.get_process_name(pid)?;

        Some(ProcessInfo { pid, name })
    }

    /// Reset priority về Normal (safe shutdown)
    #[cfg(target_os = "windows")]
    pub fn reset_priority(&mut self) -> Result<()> {
        if let Some(pid) = self.find_roblox_pid() {
            unsafe {
                let handle = OpenProcess(PROCESS_SET_INFORMATION, false, pid)
                    .context("Không thể mở process")?;

                let result = SetPriorityClass(handle, NORMAL_PRIORITY_CLASS);
                CloseHandle(handle).ok();
                result.context("Không thể reset priority")?;
            }
        }
        Ok(())
    }
}

/// Thông tin về process
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
}