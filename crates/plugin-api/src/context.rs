use std::path::{Path, PathBuf};

use crate::host::HostState;
use crate::{
    AlertThresholds, PluginCapability, PluginError, ProcessInfo, RuntimeConfig, SystemInfo,
    SystemSnapshot,
};

/// Context passed to plugin lifecycle methods.
///
/// Provides safe, limited access to application state and plugin-specific
/// directories. The live kernel state is only reachable through the
/// [`HostState`] view enforced by capability checks.
pub struct PluginContext<'a> {
    host: &'a mut dyn HostState,
    plugin_data_dir: PathBuf,
    capabilities: Vec<PluginCapability>,
}

impl<'a> PluginContext<'a> {
    /// Create a new context. Used by the kernel host.
    pub fn new(
        host: &'a mut dyn HostState,
        plugin_data_dir: PathBuf,
        capabilities: Vec<PluginCapability>,
    ) -> Self {
        Self {
            host,
            plugin_data_dir,
            capabilities,
        }
    }

    fn check_capability(&self, cap: &PluginCapability) -> Result<(), PluginError> {
        if self.capabilities.contains(cap) {
            Ok(())
        } else {
            Err(PluginError::Recoverable(format!(
                "plugin does not have required capability: {:?}",
                cap
            )))
        }
    }

    /// Full system snapshot with all available metrics.
    /// Requires `ReadSystemInfo` capability.
    pub fn snapshot(&self) -> SystemSnapshot {
        self.host.snapshot()
    }

    /// The top N processes sorted by CPU usage.
    /// Requires `ReadSystemInfo` capability.
    pub fn top_processes(&self, n: usize) -> Vec<ProcessInfo> {
        let snap = self.snapshot();
        snap.processes.into_iter().take(n).collect()
    }

    /// Kill a process by PID. Returns true if the signal was sent.
    /// Requires `KillProcesses` capability.
    pub fn kill_process(&mut self, pid: u32) -> Result<bool, PluginError> {
        self.check_capability(&PluginCapability::KillProcesses)?;
        Ok(self.host.kill_process(pid))
    }

    /// Set alert thresholds for CPU, memory, and disk.
    /// Requires `ModifyConfig` capability.
    pub fn set_alert_thresholds(
        &mut self,
        cpu: f64,
        mem: f64,
        disk: f64,
    ) -> Result<(), PluginError> {
        self.check_capability(&PluginCapability::ModifyConfig)?;
        self.host.set_alert_thresholds(cpu, mem, disk);
        Ok(())
    }

    /// Current alert thresholds.
    pub fn alerts(&self) -> AlertThresholds {
        self.host.alerts()
    }

    /// Current runtime configuration.
    pub fn config(&self) -> RuntimeConfig {
        self.host.config()
    }

    /// Switch to a theme by name. Returns true if found.
    /// Requires `ModifyConfig` capability.
    pub fn set_theme_by_name(&mut self, name: &str) -> Result<bool, PluginError> {
        self.check_capability(&PluginCapability::ModifyConfig)?;
        Ok(self.host.set_theme_by_name(name))
    }

    /// Switch to a layout by name. Returns true if found.
    /// Requires `ModifyConfig` capability.
    pub fn set_layout_by_name(&mut self, name: &str) -> Result<bool, PluginError> {
        self.check_capability(&PluginCapability::ModifyConfig)?;
        Ok(self.host.set_layout_by_name(name))
    }

    /// Set the update interval in milliseconds.
    /// Requires `ModifyConfig` capability.
    pub fn set_update_interval(&mut self, ms: u64) -> Result<(), PluginError> {
        self.check_capability(&PluginCapability::ModifyConfig)?;
        self.host.set_update_interval_ms(ms);
        Ok(())
    }

    /// Current system info (hostname, OS, kernel).
    /// Requires `ReadSystemInfo` capability.
    pub fn system_info(&self) -> SystemInfo {
        self.host.system_info()
    }

    /// Plugin-specific data directory (`~/.config/xtop/plugins/<plugin_id>/`).
    pub fn data_dir(&self) -> &Path {
        &self.plugin_data_dir
    }
}
