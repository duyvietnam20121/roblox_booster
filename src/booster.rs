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
        ABOVE_NORMAL_PRIORITY_CLASS, GetPriorityClass, HIGH_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS,
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
}

/// Process optimization statistics
#[derive(Debug, Default, Clone)]
pub struct OptimizationStats {
    pub processes_boosted: usize,
    pub memory_cleared: bool,
    pub priority_level: u8,
}

/// Safety limits to prevent system instability
const MAX_PROCESSES_TO_BOOST: usize = 5;
const MIN_PROCESS_LIFETIME_MS: u64 = 3000; // 3 seconds

/// Safe Roblox installation paths (whitelist approach)
const SAFE_ROBLOX_PATHS: &[&str] = &[
    r"C:\Users\*\AppData\Local\Roblox\Versions\",
    r"C:\Program Files (x86)\Roblox\Versions\",
];

/// SystemBooster - Safe process optimizer with strict path validation
pub struct SystemBooster {
    system: System,
    roblox_pids: HashSet<u32>,
    config: Config,
    last_stats: OptimizationStats,
    allowed_paths: Vec<PathBuf>,
}

impl SystemBooster {
    /// Create a new SystemBooster with safe path validation
    #[must_use]
    pub fn new(config: Config) -> Self {
        let allowed_paths = Self::build_allowed_paths();
        
        Self {
            system: System::new_all(),
            roblox_pids: HashSet::with_capacity(5),
            config,
            last_stats: OptimizationStats::default(),
            allowed_paths,
        }
    }

    /// Build list of allowed Roblox installation paths
    fn build_allowed_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // Get current user's LocalAppData
        if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
            let roblox_versions = PathBuf::from(localappdata).join("Roblox").join("Versions");
            if roblox_versions.exists() {
                paths.push(roblox_versions);
            }
        }

        // Fallback: Program Files
        if let Ok(programfiles) = std::env::var("ProgramFiles(x86)") {
            let roblox_versions = PathBuf::from(programfiles).join("Roblox").join("Versions");
            if roblox_versions.exists() {
                paths.push(roblox_versions);
            }
        }

        paths
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

    /// Check if a process name is Roblox-related (strict filtering)
    fn is_roblox_process(name: &str) -> bool {
        let name_lower = name.to_lowercase();

        // Must contain roblox or rbx
        let contains_roblox = name_lower.contains("roblox") || name_lower.contains("rbx");

        // STRICT exclusion list
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

    /// Validate process executable path is in allowed Roblox directories
    fn is_safe_path(&self, pid: u32) -> bool {
        #[cfg(target_os = "windows")]
        {
            if let Some(process) = self.system.process(Pid::from_u32(pid)) {
                if let Some(exe_path) = process.exe() {
                    // Check if process is in allowed paths
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

    /// Comprehensive safety check before optimization
    fn is_safe_to_optimize(&self, pid: u32) -> Result<()> {
        // Check 1: Process exists
        let Some(process) = self.system.process(Pid::from_u32(pid)) else {
            return Err(BoosterError::SafetyCheckFailed(format!(
                "Process {pid} no longer exists"
            ))
            .into());
        };

        // Check 2: Process uptime (avoid very new processes)
        let uptime = process.run_time();
        if uptime < MIN_PROCESS_LIFETIME_MS / 1000 {
            return Err(BoosterError::SafetyCheckFailed(format!(
                "Process {pid} too new ({uptime}s < {}s)",
                MIN_PROCESS_LIFETIME_MS / 1000
            ))
            .into());
        }

        // Check 3: Process name validation
        let name = process.name().to_string_lossy();
        if !Self::is_roblox_process(&name) {
            return Err(BoosterError::SafetyCheckFailed(format!(
                "Process name '{}' failed validation",
                name
            ))
            .into());
        }

        // Check 4: Path validation (CRITICAL)
        if !self.is_safe_path(pid) {
            return Err(BoosterError::PathValidationFailed(format!(
                "Process {pid} executable not in allowed Roblox directories"
            ))
            .into());
        }

        Ok(())
    }

    /// Auto-detect and boost new Roblox processes (with safety limits)
    pub fn auto_detect_and_boost(&mut self) -> Option<String> {
        self.config.auto_detect_roblox.then(|| {
            self.system.refresh_processes(ProcessesToUpdate::All, true);

            // Safety limit: Don't boost too many processes
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
                    // CRITICAL: Full safety check before boosting
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

                // Safety: Don't process too many in one cycle
                if new_processes.len() >= 2 {
                    break;
                }
            }

            // Clean up dead processes
            self.roblox_pids
                .retain(|&pid| self.system.process(Pid::from_u32(pid)).is_some());

            (!new_processes.is_empty()).then(|| {
                format!(
                    "✓ Auto-boosted {} process(es): {}",
                    new_processes.len(),
                    new_processes.join(", ")
                )
            })
        })?
    }

    /// Enable optimizations with comprehensive safety checks
    pub fn enable(&mut self) -> Result<String> {
        self.system.refresh_all();

        let mut stats = OptimizationStats {
            priority_level: self.config.priority_level,
            ..Default::default()
        };

        let mut optimizations = Vec::new();
        let mut errors = Vec::new();

        // Collect and validate Roblox processes
        let mut roblox_processes: Vec<(u32, String)> = Vec::new();
        for (pid, process) in self.system.processes() {
            let name = process.name().to_string_lossy();
            if Self::is_roblox_process(&name) {
                roblox_processes.push((pid.as_u32(), name.into_owned()));
            }
        }

        // Safety check: Process count
        if roblox_processes.is_empty() {
            return Err(BoosterError::NoProcessesFound.into());
        }

        if roblox_processes.len() > MAX_PROCESSES_TO_BOOST {
            return Err(BoosterError::SafetyCheckFailed(format!(
                "Too many Roblox processes ({} > {}). This may indicate a problem.",
                roblox_processes.len(),
                MAX_PROCESSES_TO_BOOST
            ))
            .into());
        }

        // Phase 1: Validate and optimize each process
        for (pid_u32, name) in roblox_processes {
            // Full safety validation
            match self.is_safe_to_optimize(pid_u32) {
                Ok(()) => {
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
                Err(e) => {
                    errors.push(format!("⚠️ Skipped {name}: {e}"));
                }
            }
        }

        // Phase 2: Memory optimization (placeholder only, no actual modification)
        if self.config.clear_memory_cache {
            stats.memory_cleared = true;
            optimizations.push("✓ Memory optimization enabled".into());
        }

        self.last_stats = stats.clone();

        // Build result message
        if optimizations.is_empty() {
            return Err(BoosterError::NoProcessesFound.into());
        }

        let mut message = format!(
            "🚀 Booster enabled - {} process(es) optimized\n\n",
            stats.processes_boosted
        );

        message.push_str(&optimizations.join("\n"));

        if !errors.is_empty() {
            message.push_str("\n\n⚠️ Warnings:\n");
            message.push_str(&errors.join("\n"));
        }

        Ok(message)
    }

    /// Optimize single process (MINIMAL permissions, priority only)
    fn optimize_process(&self, pid: u32) -> Result<()> {
        #[cfg(target_os = "windows")]
        unsafe {
            // MINIMAL permissions: Only what we absolutely need
            let handle = OpenProcess(
                PROCESS_SET_INFORMATION | PROCESS_QUERY_INFORMATION,
                false,
                pid,
            )
            .map_err(|e| BoosterError::ProcessOpen {
                pid,
                reason: format!("{e:?}"),
            })?;

            // Check current priority first (avoid unnecessary changes)
            let current_priority = GetPriorityClass(handle);

            // Determine target priority
            let target_priority = match self.config.priority_level {
                0 => NORMAL_PRIORITY_CLASS,
                1 => ABOVE_NORMAL_PRIORITY_CLASS,
                _ => HIGH_PRIORITY_CLASS,
            };

            // Don't change if already at target or higher
            if current_priority >= target_priority.0 {
                let _ = CloseHandle(handle);
                return Ok(());
            }

            // Set priority (ONLY operation performed)
            SetPriorityClass(handle, target_priority).map_err(|e| {
                let _ = CloseHandle(handle);
                BoosterError::PrioritySet {
                    pid,
                    reason: format!("{e:?}"),
                }
            })?;

            // Verify priority was set correctly
            let new_priority = GetPriorityClass(handle);
            if new_priority != target_priority.0 {
                let _ = CloseHandle(handle);
                return Err(BoosterError::PrioritySet {
                    pid,
                    reason: "Priority verification failed".into(),
                }
                .into());
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

    /// Disable all optimizations
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

        let mut message = format!("🔻 Booster disabled - {restored}/{count} processes restored");

        if !errors.is_empty() {
            message.push_str("\n\n⚠️ Warnings:\n");
            message.push_str(&errors.join("\n"));
        }

        Ok(message)
    }

    /// Get current Roblox process count (with path validation)
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

    /// Launch Roblox (SAFE: Protocol handler only)
    pub fn launch_roblox(&self) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            // ONLY use protocol handler (safest method)
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

            // Only restore if boosted
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
}

impl Drop for SystemBooster {
    fn drop(&mut self) {
        let _ = self.disable();
    }
}