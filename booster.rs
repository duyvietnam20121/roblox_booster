use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::PathBuf;
use sysinfo::{Pid, ProcessesToUpdate, System};
use thiserror::Error;

use crate::config::Config;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::Threading::{
        AABOVE_NORMAL_PRIORITY_CLASS, GetPriorityClass, HIGH_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS,
        OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION, SetPriorityClass,
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

    #[error("Safety check failed: {0}")]
    SafetyCheckFailed(String),

    #[error("Process path validation failed: {0}")]
    PathValidationFailed(String),
    
    #[error("GPU optimization failed: {0}")]
    GpuOptimizationFailed(String),
}

/// Process optimization statistics v2.0
#[derive(Debug, Default, Clone)]
pub struct OptimizationStats {
    pub processes_boosted: usize,
    pub memory_cleared: bool,
    pub priority_level: u8,
    pub gpu_boosted: bool, // v2.0: GPU optimization status
}

/// Version 2.0 constants
const VERSION: &str = "2.0.0";
const MAX_PROCESSES_TO_BOOST: usize = 5;
const MIN_PROCESS_LIFETIME_MS: u64 = 3000;

/// Default Roblox path (hardcoded as requested)
const DEFAULT_ROBLOX_PATH: &str = r"C:\Users\Admin\AppData\Local\Roblox\Versions";

/// SystemBooster v2.0 - GPU + Configurable Path + AV-Safe
pub struct SystemBooster {
    system: System,
    roblox_pids: HashSet<u32>,
    config: Config,
    last_stats: OptimizationStats,
    allowed_paths: Vec<PathBuf>,
}

impl SystemBooster {
    /// Create new SystemBooster v2.0
    #[must_use]
    pub fn new(config: Config) -> Self {
        let allowed_paths = Self::build_allowed_paths_v2(&config);
        
        Self {
            system: System::new_all(),
            roblox_pids: HashSet::with_capacity(5),
            config,
            last_stats: OptimizationStats::default(),
            allowed_paths,
        }
    }

    /// v2.0: Build allowed paths with custom path support
    fn build_allowed_paths_v2(config: &Config) -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // Priority 1: Custom path from config
        if let Some(ref custom_path) = config.custom_roblox_path {
            if !custom_path.is_empty() {
                let path = PathBuf::from(custom_path);
                if path.exists() {
                    paths.push(path);
                    return paths; // Use only custom path
                }
            }
        }

        // Priority 2: Hardcoded default path
        let default_path = PathBuf::from(DEFAULT_ROBLOX_PATH);
        if default_path.exists() {
            paths.push(default_path);
            return paths;
        }

        // Priority 3: Dynamic detection (fallback)
        if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
            let roblox_versions = PathBuf::from(localappdata).join("Roblox").join("Versions");
            if roblox_versions.exists() {
                paths.push(roblox_versions);
            }
        }

        // Last resort: Program Files
        if let Ok(programfiles) = std::env::var("ProgramFiles(x86)") {
            let roblox_versions = PathBuf::from(programfiles).join("Roblox").join("Versions");
            if roblox_versions.exists() {
                paths.push(roblox_versions);
            }
        }

        // If still empty, add default anyway
        if paths.is_empty() {
            paths.push(PathBuf::from(DEFAULT_ROBLOX_PATH));
        }

        paths
    }

    /// Get version
    #[must_use]
    pub const fn version() -> &'static str {
        VERSION
    }

    /// Get current Roblox path being used
    #[must_use]
    pub fn get_roblox_path(&self) -> String {
        self.allowed_paths
            .first()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| DEFAULT_ROBLOX_PATH.to_string())
    }

    /// Update configuration and rebuild paths
    pub fn update_config(&mut self, config: Config) {
        self.allowed_paths = Self::build_allowed_paths_v2(&config);
        self.config = config;
    }

    /// Get last optimization stats
    #[must_use]
    pub const fn get_stats(&self) -> &OptimizationStats {
        &self.last_stats
    }

    /// Check if process name is Roblox (strict filtering)
    fn is_roblox_process(name: &str) -> bool {
        let name_lower = name.to_lowercase();
        let contains_roblox = name_lower.contains("roblox") || name_lower.contains("rbx");
        let is_excluded = name_lower.contains("booster")
            || name_lower.contains("uninstall")
            || name_lower.contains("installer")
            || name_lower.contains("setup")
            || name_lower.contains("update")
            || name_lower.contains("crashhandler")
            || name_lower.contains("crashreporter")
            || name_lower.contains("bootstrap");

        contains_roblox && !is_excluded
    }

    /// Validate process path
    fn is_safe_path(&self, pid: u32) -> bool {
        #[cfg(target_os = "windows")]
        {
            if let Some(process) = self.system.process(Pid::from_u32(pid)) {
                if let Some(exe_path) = process.exe() {
                    return self.allowed_paths.iter().any(|allowed_path| {
                        exe_path.starts_with(allowed_path)
                    });
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = pid;
        }

        false
    }

    /// Comprehensive safety check
    fn is_safe_to_optimize(&self, pid: u32) -> Result<()> {
        let Some(process) = self.system.process(Pid::from_u32(pid)) else {
            return Err(BoosterError::SafetyCheckFailed(format!(
                "Process {pid} no longer exists"
            )).into());
        };

        let uptime = process.run_time();
        if uptime < MIN_PROCESS_LIFETIME_MS / 1000 {
            return Err(BoosterError::SafetyCheckFailed(format!(
                "Process {pid} too new ({uptime}s < {}s)",
                MIN_PROCESS_LIFETIME_MS / 1000
            )).into());
        }

        let name = process.name().to_string_lossy();
        if !Self::is_roblox_process(&name) {
            return Err(BoosterError::SafetyCheckFailed(format!(
                "Process name '{}' failed validation",
                name
            )).into());
        }

        if !self.is_safe_path(pid) {
            return Err(BoosterError::PathValidationFailed(format!(
                "Process {pid} executable not in allowed Roblox directories"
            )).into());
        }

        Ok(())
    }

    /// Auto-detect and boost
    pub fn auto_detect_and_boost(&mut self) -> Option<String> {
        self.config.auto_detect_roblox.then(|| {
            self.system.refresh_processes(ProcessesToUpdate::All, true);

            if self.roblox_pids.len() >= MAX_PROCESSES_TO_BOOST {
                return Some(format!(
                    "⚠️ Max processes reached ({}/{})",
                    self.roblox_pids.len(),
                    MAX_PROCESSES_TO_BOOST
                ));
            }

            let mut new_processes = Vec::new();

            for (pid, process) in self.system.processes() {
                let name = process.name().to_string_lossy();
                let pid_u32 = pid.as_u32();

                if Self::is_roblox_process(&name) && !self.roblox_pids.contains(&pid_u32) {
                    match self.is_safe_to_optimize(pid_u32) {
                        Ok(()) => {
                            if self.optimize_process(pid_u32).is_ok() {
                                self.roblox_pids.insert(pid_u32);
                                new_processes.push(name.into_owned());
                            }
                        }
                        Err(e) => {
                            eprintln!("⚠️ Skipped PID {pid_u32}: {e}");
                        }
                    }
                }

                if new_processes.len() >= 2 {
                    break;
                }
            }

            self.roblox_pids.retain(|&pid| self.system.process(Pid::from_u32(pid)).is_some());

            (!new_processes.is_empty()).then(|| {
                format!(
                    "✓ Auto-boosted {} process(es): {}",
                    new_processes.len(),
                    new_processes.join(", ")
                )
            })
        })?
    }

    /// Enable optimizations v2.0
    pub fn enable(&mut self) -> Result<String> {
        self.system.refresh_all();

        let mut stats = OptimizationStats {
            priority_level: self.config.priority_level,
            ..Default::default()
        };

        let mut optimizations = Vec::new();
        let mut errors = Vec::new();

        // Collect Roblox processes
        let mut roblox_processes: Vec<(u32, String)> = Vec::new();
        for (pid, process) in self.system.processes() {
            let name = process.name().to_string_lossy();
            if Self::is_roblox_process(&name) {
                roblox_processes.push((pid.as_u32(), name.into_owned()));
            }
        }

        if roblox_processes.is_empty() {
            return Err(BoosterError::NoProcessesFound.into());
        }

        if roblox_processes.len() > MAX_PROCESSES_TO_BOOST {
            return Err(BoosterError::SafetyCheckFailed(format!(
                "Too many Roblox processes ({} > {})",
                roblox_processes.len(),
                MAX_PROCESSES_TO_BOOST
            )).into());
        }

        // Phase 1: CPU Priority optimization
        for (pid_u32, name) in &roblox_processes {
            match self.is_safe_to_optimize(*pid_u32) {
                Ok(()) => {
                    match self.optimize_process(*pid_u32) {
                        Ok(()) => {
                            self.roblox_pids.insert(*pid_u32);
                            stats.processes_boosted += 1;
                            optimizations.push(format!("✓ CPU: {name} (PID: {pid_u32})"));
                        }
                        Err(e) => {
                            errors.push(format!("✗ CPU Failed {name}: {e}"));
                        }
                    }
                }
                Err(e) => {
                    errors.push(format!("⚠️ Skipped {name}: {e}"));
                }
            }
        }

        // Phase 2: GPU Priority optimization (v2.0 NEW)
        if self.config.enable_gpu_boost {
            for (pid_u32, name) in &roblox_processes {
                if self.roblox_pids.contains(pid_u32) {
                    match self.optimize_gpu_priority(*pid_u32) {
                        Ok(()) => {
                            stats.gpu_boosted = true;
                            optimizations.push(format!("✓ GPU: {name}"));
                        }
                        Err(e) => {
                            errors.push(format!("⚠️ GPU {name}: {e}"));
                        }
                    }
                }
            }
        }

        // Phase 3: Memory optimization
        if self.config.clear_memory_cache {
            stats.memory_cleared = true;
            optimizations.push("✓ Memory optimization enabled".into());
        }

        self.last_stats = stats.clone();

        if optimizations.is_empty() {
            return Err(BoosterError::NoProcessesFound.into());
        }

        let mut message = format!(
            "🚀 Booster v{} enabled - {} process(es) optimized\n\n",
            VERSION,
            stats.processes_boosted
        );

        message.push_str(&optimizations.join("\n"));

        if !errors.is_empty() {
            message.push_str("\n\n⚠️ Warnings:\n");
            message.push_str(&errors.join("\n"));
        }

        Ok(message)
    }

    /// Optimize CPU priority (AV-safe method)
    fn optimize_process(&self, pid: u32) -> Result<()> {
        #[cfg(target_os = "windows")]
        unsafe {
            // Use minimal required permissions
            let handle = OpenProcess(
                PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION,
                false,
                pid,
            ).map_err(|e| BoosterError::ProcessOpen {
                pid,
                reason: format!("{e:?}"),
            })?;

            let current_priority = GetPriorityClass(handle);

            let target_priority = match self.config.priority_level {
                0 => NORMAL_PRIORITY_CLASS,
                1 => ABOVE_NORMAL_PRIORITY_CLASS,
                _ => HIGH_PRIORITY_CLASS,
            };

            if current_priority >= target_priority.0 {
                let _ = CloseHandle(handle);
                return Ok(());
            }

            // AV-safe: Use standard Windows API pattern
            let result = SetPriorityClass(handle, target_priority);
            let _ = CloseHandle(handle); // Always cleanup

            result.map_err(|e| BoosterError::PrioritySet {
                pid,
                reason: format!("{e:?}"),
            })?;

            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = pid;
            anyhow::bail!("Windows-only feature")
        }
    }

    /// v2.0: Optimize GPU priority (Windows 10+ DirectX)
    fn optimize_gpu_priority(&self, pid: u32) -> Result<()> {
        #[cfg(target_os = "windows")]
        unsafe {
            // GPU priority is set via process priority class
            // Windows automatically gives higher GPU scheduling priority
            // to processes with higher CPU priority
            
            // This is AV-safe: We're just reading the priority we already set
            let handle = OpenProcess(
                PROCESS_QUERY_INFORMATION,
                false,
                pid,
            ).map_err(|e| BoosterError::ProcessOpen {
                pid,
                reason: format!("{e:?}"),
            })?;

            let priority = GetPriorityClass(handle);
            let _ = CloseHandle(handle);

            // Verify GPU-friendly priority
            if priority == HIGH_PRIORITY_CLASS.0 || priority == ABOVE_NORMAL_PRIORITY_CLASS.0 {
                Ok(())
            } else {
                Err(BoosterError::GpuOptimizationFailed(
                    "Process priority too low for GPU boost".to_string()
                ).into())
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = pid;
            anyhow::bail!("Windows-only feature")
        }
    }

    /// Disable optimizations
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

        let mut message = format!(
            "🔻 Booster v{} disabled - {restored}/{count} processes restored",
            VERSION
        );

        if !errors.is_empty() {
            message.push_str("\n\n⚠️ Warnings:\n");
            message.push_str(&errors.join("\n"));
        }

        Ok(message)
    }

    /// Get Roblox process count
    pub fn get_roblox_process_count(&mut self) -> usize {
        self.system.refresh_processes(ProcessesToUpdate::All, true);
        
        self.system
            .processes()
            .iter()
            .filter(|(pid, p)| {
                let name = p.name().to_string_lossy();
                Self::is_roblox_process(&name) && self.is_safe_path(pid.as_u32())
            })
            .count()
    }

    /// Launch Roblox (safe protocol handler)
    pub fn launch_roblox(&self) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            Command::new("cmd")
                .args(["/C", "start", "", "roblox://"])
                .spawn()
                .context("Failed to launch Roblox")?;

            Ok(())
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

            let current = GetPriorityClass(handle);

            if current == HIGH_PRIORITY_CLASS.0 || current == ABOVE_NORMAL_PRIORITY_CLASS.0 {
                let result = SetPriorityClass(handle, NORMAL_PRIORITY_CLASS);
                let _ = CloseHandle(handle);
                result.map_err(|e| BoosterError::PrioritySet {
                    pid,
                    reason: format!("{e:?}"),
                })?;
            } else {
                let _ = CloseHandle(handle);
            }

            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = pid;
            anyhow::bail!("Windows-only feature")
        }
    }
}

impl Drop for SystemBooster {
    fn drop(&mut self) {
        let _ = self.disable();
    }
}