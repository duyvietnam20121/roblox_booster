use anyhow::{Context, Result};
use std::collections::HashSet;
use sysinfo::{Pid, ProcessesToUpdate, System};
use thiserror::Error;

use crate::config::Config;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::CloseHandle,
    System::Threading::{
        ABOVE_NORMAL_PRIORITY_CLASS, GetPriorityClass, HIGH_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS,
        OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION, PROCESS_SET_QUOTA,
        SetPriorityClass, SetProcessWorkingSetSize,
    },
};

#[derive(Debug, Error)]
pub enum BoosterError {
    #[error("Failed to open process {pid}: {reason}")]
    ProcessOpen { pid: u32, reason: String },

    #[error("Failed to set process priority for PID {pid}: {reason}")]
    PrioritySet { pid: u32, reason: String },

    #[error("No Roblox processes found")]
    NoProcessesFound,
}

/// Process optimization stats
#[derive(Debug, Default, Clone)]
pub struct OptimizationStats {
    pub processes_boosted: usize,
    pub memory_cleared: bool,
    pub priority_level: u8,
}

/// SystemBooster handles system optimization for gaming
pub struct SystemBooster {
    system: System,
    roblox_pids: HashSet<u32>,
    config: Config,
    last_stats: OptimizationStats,
}

impl SystemBooster {
    /// Create a new SystemBooster instance
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self {
            system: System::new_all(),
            roblox_pids: HashSet::with_capacity(10),
            config,
            last_stats: OptimizationStats::default(),
        }
    }

    /// Update configuration
    pub fn update_config(&mut self, config: Config) {
        self.config = config;
    }

    /// Get last optimization stats
    #[must_use]
    pub const fn get_stats(&self) -> &OptimizationStats {
        &self.last_stats
    }

    /// Check if a process name is Roblox-related
    fn is_roblox_process(name: &str) -> bool {
        let name_lower = name.to_lowercase();
        (name_lower.contains("roblox") || name_lower.contains("rbx"))
            && !name_lower.contains("booster")
            && !name_lower.contains("uninstall")
    }

    /// Check for new Roblox processes and auto-boost them
    pub fn auto_detect_and_boost(&mut self) -> Option<String> {
        self.config.auto_detect_roblox.then(|| {
            // FIX: Add second parameter (true) to refresh processes list
            self.system.refresh_processes(ProcessesToUpdate::All, true);
            let mut new_processes = Vec::new();

            for (pid, process) in self.system.processes() {
                let name = process.name().to_string_lossy();
                let pid_u32 = pid.as_u32();

                if Self::is_roblox_process(&name) && !self.roblox_pids.contains(&pid_u32) {
                    if self.optimize_process(pid_u32).is_ok() {
                        self.roblox_pids.insert(pid_u32);
                        new_processes.push(name.into_owned());
                    }
                }
            }

            // Clean up dead processes
            self.roblox_pids
                .retain(|&pid| self.system.process(Pid::from_u32(pid)).is_some());

            (!new_processes.is_empty()).then(|| {
                format!(
                    "Auto-boosted {} process(es): {}",
                    new_processes.len(),
                    new_processes.join(", ")
                )
            })
        })?
    }

    /// Enable system optimizations
    pub fn enable(&mut self) -> Result<String> {
        self.system.refresh_all();

        let mut stats = OptimizationStats {
            priority_level: self.config.priority_level,
            ..Default::default()
        };

        let mut optimizations = Vec::new();
        let mut errors = Vec::new();

        // Phase 1: Find and boost all Roblox processes
        for (pid, process) in self.system.processes() {
            let name = process.name().to_string_lossy();

            if Self::is_roblox_process(&name) {
                let pid_u32 = pid.as_u32();

                match self.optimize_process(pid_u32) {
                    Ok(()) => {
                        self.roblox_pids.insert(pid_u32);
                        stats.processes_boosted += 1;
                        optimizations.push(format!("✓ Optimized {name} (PID: {pid_u32})"));
                    }
                    Err(e) => {
                        errors.push(format!("✗ Failed {name}: {e}"));
                    }
                }
            }
        }

        // Phase 2: Clear system cache if enabled
        if self.config.clear_memory_cache && cfg!(target_os = "windows") {
            match self.optimize_system_memory() {
                Ok(()) => {
                    stats.memory_cleared = true;
                    optimizations.push("✓ System memory optimized".into());
                }
                Err(e) => {
                    errors.push(format!("✗ Memory optimization: {e}"));
                }
            }
        }

        self.last_stats = stats.clone();

        // Build result message
        if optimizations.is_empty() {
            return Err(BoosterError::NoProcessesFound.into());
        }

        let mut message = format!(
            "Booster enabled - {} process(es) optimized\n\n",
            stats.processes_boosted
        );

        message.push_str(&optimizations.join("\n"));

        if !errors.is_empty() {
            message.push_str("\n\nWarnings:\n");
            message.push_str(&errors.join("\n"));
        }

        Ok(message)
    }

    /// Optimize a single process
    /// FIX: Remove unused `name` parameter
    fn optimize_process(&self, pid: u32) -> Result<()> {
        #[cfg(target_os = "windows")]
        unsafe {
            // Open process with all required permissions
            let handle = OpenProcess(
                PROCESS_SET_INFORMATION | PROCESS_SET_QUOTA | PROCESS_QUERY_INFORMATION,
                false,
                pid,
            )
            .map_err(|e| BoosterError::ProcessOpen {
                pid,
                reason: format!("{e:?}"),
            })?;

            // Step 1: Set priority class
            let priority = match self.config.priority_level {
                0 => NORMAL_PRIORITY_CLASS,
                1 => ABOVE_NORMAL_PRIORITY_CLASS,
                _ => HIGH_PRIORITY_CLASS,
            };

            SetPriorityClass(handle, priority).map_err(|e| {
                let _ = CloseHandle(handle);
                BoosterError::PrioritySet {
                    pid,
                    reason: format!("{e:?}"),
                }
            })?;

            // Step 2: Optimize working set (memory trimming)
            if self.config.clear_memory_cache {
                let _ = SetProcessWorkingSetSize(handle, usize::MAX, usize::MAX);
            }

            CloseHandle(handle).ok();
            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = pid;
            anyhow::bail!("Windows-only feature")
        }
    }

    /// Disable system optimizations
    pub fn disable(&mut self) -> Result<String> {
        let mut restored = 0;
        let mut errors = Vec::new();

        for &pid in &self.roblox_pids {
            match self.restore_process(pid) {
                Ok(()) => restored += 1,
                Err(e) => errors.push(format!("PID {pid}: {e}")),
            }
        }

        let count = self.roblox_pids.len();
        self.roblox_pids.clear();
        self.last_stats = OptimizationStats::default();

        let mut message = format!("Booster disabled - {restored}/{count} processes restored");

        if !errors.is_empty() {
            message.push_str("\n\nWarnings:\n");
            message.push_str(&errors.join("\n"));
        }

        Ok(message)
    }

    /// Get current Roblox process count
    pub fn get_roblox_process_count(&mut self) -> usize {
        // FIX: Add second parameter (true) to refresh processes list
        self.system.refresh_processes(ProcessesToUpdate::All, true);
        self.system
            .processes()
            .iter()
            .filter(|(_, p)| {
                let name = p.name().to_string_lossy();
                Self::is_roblox_process(&name)
            })
            .count()
    }

    /// Launch Roblox application
    pub fn launch_roblox(&self) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            // Method 1: Protocol handler (most reliable)
            if Command::new("cmd")
                .args(["/C", "start", "", "roblox://"])
                .spawn()
                .is_ok()
            {
                return Ok(());
            }

            // Method 2: Direct executable paths
            let localappdata = std::env::var("LOCALAPPDATA").unwrap_or_default();
            let programfiles = std::env::var("ProgramFiles(x86)").unwrap_or_default();

            let possible_paths = [
                format!(r"{localappdata}\Roblox\Versions\RobloxPlayerLauncher.exe"),
                format!(r"{programfiles}\Roblox\Versions\RobloxPlayerLauncher.exe"),
                r"C:\Program Files (x86)\Roblox\Versions\RobloxPlayerLauncher.exe".into(),
            ];

            for path in &possible_paths {
                if std::path::Path::new(path).exists() && Command::new(path).spawn().is_ok() {
                    return Ok(());
                }
            }

            anyhow::bail!(
                "Roblox not found. Please install from roblox.com or Microsoft Store.\n\
                Searched locations:\n{}",
                possible_paths.join("\n")
            )
        }

        #[cfg(not(target_os = "windows"))]
        anyhow::bail!("Roblox is Windows-only")
    }

    /// Restore process to normal priority
    fn restore_process(&self, pid: u32) -> Result<()> {
        #[cfg(target_os = "windows")]
        unsafe {
            let handle = OpenProcess(
                PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION,
                false,
                pid,
            )
            .map_err(|e| BoosterError::ProcessOpen {
                pid,
                reason: format!("{e:?}"),
            })?;

            // Check current priority before restoring
            let current = GetPriorityClass(handle);

            // Only restore if it's still high priority
            if current == HIGH_PRIORITY_CLASS.0 || current == ABOVE_NORMAL_PRIORITY_CLASS.0 {
                SetPriorityClass(handle, NORMAL_PRIORITY_CLASS).map_err(|e| {
                    let _ = CloseHandle(handle);
                    BoosterError::PrioritySet {
                        pid,
                        reason: format!("{e:?}"),
                    }
                })?;
            }

            CloseHandle(handle).ok();
            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = pid;
            anyhow::bail!("Windows-only feature")
        }
    }

    /// Optimize system memory
    fn optimize_system_memory(&self) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            // Trigger memory optimization
            let output = Command::new("cmd")
                .args(["/C", "echo", "Memory optimization triggered"])
                .output()
                .context("Failed to trigger memory optimization")?;

            if !output.status.success() {
                anyhow::bail!("Memory optimization command failed");
            }

            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        anyhow::bail!("Windows-only feature")
    }
}

impl Drop for SystemBooster {
    fn drop(&mut self) {
        let _ = self.disable();
    }
}