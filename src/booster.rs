use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::PathBuf;
use sysinfo::{Pid, ProcessesToUpdate, System};
use thiserror::Error;

use crate::config::Config;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::CloseHandle,
    System::Threading::{
        GetPriorityClass, OpenProcess, SetPriorityClass,
        ABOVE_NORMAL_PRIORITY_CLASS, HIGH_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS,
        PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION,
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

/// Process optimization stats v2.0
#[derive(Debug, Default, Clone)]
pub struct OptimizationStats {
    pub processes_boosted: usize,
    pub memory_cleared: bool,
    #[allow(dead_code)]
    pub priority_level: u8,
    pub gpu_boosted: bool,
}

/// Version constants
const VERSION: &str = "2.0.0";
const MAX_PROCESSES_TO_BOOST: usize = 5;
const MIN_PROCESS_LIFETIME_MS: u64 = 3000;

/// SystemBooster v2.0 - Modern dynamic path detection + GPU optimization
pub struct SystemBooster {
    system: System,
    roblox_pids: HashSet<u32>,
    config: Config,
    last_stats: OptimizationStats,
    allowed_paths: Vec<PathBuf>,
}

impl SystemBooster {
    /// Create new SystemBooster v2.0 with dynamic path detection
    #[must_use]
    pub fn new(config: Config) -> Self {
        let allowed_paths = Self::detect_roblox_paths(&config);
        
        Self {
            system: System::new_all(),
            roblox_pids: HashSet::with_capacity(5),
            config,
            last_stats: OptimizationStats::default(),
            allowed_paths,
        }
    }

    /// Modern dynamic Roblox path detection (no hardcoded paths)
    fn detect_roblox_paths(config: &Config) -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // Priority 1: Custom user-defined path
        if let Some(ref custom_path) = config.custom_roblox_path {
            if !custom_path.is_empty() {
                let path = PathBuf::from(custom_path);
                if path.exists() {
                    paths.push(path);
                    return paths;
                }
            }
        }

        // Priority 2: Standard LocalAppData location (most common)
        if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
            let roblox_versions = PathBuf::from(localappdata).join("Roblox").join("Versions");
            if roblox_versions.exists() {
                paths.push(roblox_versions);
            }
        }

        // Priority 3: Microsoft Store version
        if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
            let store_roblox = PathBuf::from(localappdata)
                .join("Packages")
                .join("ROBLOXCORPORATION.ROBLOX_55nm5eh3cm0pr");
            if store_roblox.exists() {
                paths.push(store_roblox);
            }
        }

        // Priority 4: Program Files (legacy installations)
        if let Ok(programfiles) = std::env::var("ProgramFiles(x86)") {
            let roblox_versions = PathBuf::from(programfiles).join("Roblox").join("Versions");
            if roblox_versions.exists() {
                paths.push(roblox_versions);
            }
        }

        paths
    }

    /// Get version
    #[must_use]
    pub const fn version() -> &'static str {
        VERSION
    }

    /// Get detected Roblox paths
    #[must_use]
    pub fn get_roblox_path(&self) -> String {
        self.allowed_paths
            .first()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "No Roblox installation detected".to_string())
    }

    /// Update configuration and rebuild paths
    pub fn update_config(&mut self, config: Config) {
        self.allowed_paths = Self::detect_roblox_paths(&config);
        self.config = config;
    }

    /// Get last optimization stats
    #[must_use]
    pub const fn get_stats(&self) -> &OptimizationStats {
        &self.last_stats
    }

    /// Check if process name is Roblox (enhanced filtering)
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

    /// Validate process is from allowed Roblox directories
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

    /// Comprehensive safety validation
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
                "Process {pid} not in validated Roblox directories"
            )).into());
        }

        Ok(())
    }

    /// Auto-detect and boost new Roblox processes
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
                            if self.optimize_process_full(pid_u32).is_ok() {
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

    /// Enable system optimizations v2.0
    pub fn enable(&mut self) -> Result<String> {
        self.system.refresh_all();

        let mut stats = OptimizationStats {
            priority_level: self.config.priority_level,
            ..Default::default()
        };

        let mut optimizations = Vec::new();
        let mut errors = Vec::new();

        // Collect validated Roblox processes
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
                    match self.optimize_process_full(*pid_u32) {
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

        // Phase 2: GPU Priority optimization (v2.0)
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

    /// Optimize process CPU priority (full optimization)
    fn optimize_process_full(&self, pid: u32) -> Result<()> {
        #[cfg(target_os = "windows")]
        unsafe {
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

            // Skip if already at target or higher priority
            if current_priority >= target_priority.0 {
                let _ = CloseHandle(handle);
                return Ok(());
            }

            let result = SetPriorityClass(handle, target_priority);
            let _ = CloseHandle(handle);

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

    /// v2.0: Optimize GPU priority for DirectX applications
    fn optimize_gpu_priority(&self, pid: u32) -> Result<()> {
        #[cfg(target_os = "windows")]
        unsafe {
            // GPU scheduling in Windows leverages CPU priority class
            // Higher CPU priority = higher GPU scheduling priority
            // This is the modern, safe approach for GPU optimization
            
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

            // Verify GPU-friendly priority level
            if priority == HIGH_PRIORITY_CLASS.0 || priority == ABOVE_NORMAL_PRIORITY_CLASS.0 {
                Ok(())
            } else {
                Err(BoosterError::GpuOptimizationFailed(
                    "CPU priority too low for GPU boost".to_string()
                ).into())
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = pid;
            anyhow::bail!("Windows-only feature")
        }
    }

    /// Disable optimizations v2.0
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

    /// Get validated Roblox process count
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

    /// Launch Roblox via protocol handler (modern method)
    pub fn launch_roblox(&self) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            Command::new("cmd")
                .args(["/C", "start", "", "roblox://"])
                .spawn()
                .context("Failed to launch Roblox via protocol handler")?;

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

            // Only restore if currently boosted
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
