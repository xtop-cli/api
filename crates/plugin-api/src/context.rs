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
    ///
    /// Requires the `ReadSystemInfo` capability. Returns
    /// [`PluginError::Recoverable`] when the plugin did not declare it.
    pub fn snapshot(&self) -> Result<SystemSnapshot, PluginError> {
        self.check_capability(&PluginCapability::ReadSystemInfo)?;
        Ok(self.host.snapshot())
    }

    /// The top `n` processes by CPU usage, highest first.
    ///
    /// The contract sorts the snapshot's processes by `cpu_usage`
    /// descending before taking the first `n`, so the guarantee does not
    /// depend on the ordering produced by the data source.
    ///
    /// Requires the `ReadSystemInfo` capability. Returns
    /// [`PluginError::Recoverable`] when the plugin did not declare it.
    pub fn top_processes(&self, n: usize) -> Result<Vec<ProcessInfo>, PluginError> {
        let snap = self.snapshot()?;
        let mut processes = snap.processes;
        processes.sort_by(|a, b| b.cpu_usage.total_cmp(&a.cpu_usage));
        processes.truncate(n);
        Ok(processes)
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
    ///
    /// Requires the `ReadSystemInfo` capability. Returns
    /// [`PluginError::Recoverable`] when the plugin did not declare it.
    pub fn system_info(&self) -> Result<SystemInfo, PluginError> {
        self.check_capability(&PluginCapability::ReadSystemInfo)?;
        Ok(self.host.system_info())
    }

    /// Plugin-specific data directory.
    ///
    /// The path is host-provided: the kernel decides where plugin data lives
    /// and passes it into [`PluginContext::new`]. The contract deliberately
    /// does not prescribe a concrete location, so it cannot drift from what
    /// the host actually hands out.
    pub fn data_dir(&self) -> &Path {
        &self.plugin_data_dir
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::capability::PluginCapability;
    use crate::context::PluginContext;
    use crate::host::{AlertThresholds, HostState, RuntimeConfig};
    use crate::model::{LoadAvg, MemoryInfo, ProcessInfo, SwapInfo, SystemInfo, SystemSnapshot};
    use crate::PluginError;

    /// Minimal host double: serves back the snapshot it was built with.
    struct FakeHost {
        snapshot: SystemSnapshot,
    }

    impl FakeHost {
        fn new(processes: Vec<ProcessInfo>) -> Self {
            Self {
                snapshot: SystemSnapshot {
                    cpus: vec![],
                    memory: MemoryInfo {
                        total: 0,
                        used: 0,
                        available: 0,
                        free: 0,
                        percent: 0.0,
                    },
                    swap: SwapInfo {
                        total: 0,
                        used: 0,
                        free: 0,
                        percent: 0.0,
                    },
                    disks: vec![],
                    networks: vec![],
                    processes,
                    load_avg: LoadAvg {
                        one: 0.0,
                        five: 0.0,
                        fifteen: 0.0,
                    },
                    uptime: 0,
                    cpu_temp: 0.0,
                    disk_io: vec![],
                    batteries: vec![],
                    gpus: vec![],
                    sys_info: SystemInfo::default(),
                },
            }
        }
    }

    impl HostState for FakeHost {
        fn snapshot(&self) -> SystemSnapshot {
            self.snapshot.clone()
        }

        fn system_info(&self) -> SystemInfo {
            self.snapshot.sys_info.clone()
        }

        fn kill_process(&mut self, _pid: u32) -> bool {
            false
        }

        fn set_alert_thresholds(&mut self, _cpu: f64, _mem: f64, _disk: f64) {}

        fn alerts(&self) -> AlertThresholds {
            AlertThresholds {
                cpu_high: 0.0,
                mem_high: 0.0,
                disk_high: 0.0,
            }
        }

        fn config(&self) -> RuntimeConfig {
            RuntimeConfig {
                theme: String::new(),
                layout: String::new(),
                interval_ms: 0,
                hostname: String::new(),
            }
        }

        fn set_theme_by_name(&mut self, _name: &str) -> bool {
            false
        }

        fn set_layout_by_name(&mut self, _name: &str) -> bool {
            false
        }

        fn set_update_interval_ms(&mut self, _ms: u64) {}
    }

    /// Minimal `ProcessInfo` with only pid/cpu set.
    fn process(pid: u32, cpu_usage: f64) -> ProcessInfo {
        ProcessInfo {
            pid,
            name: String::new(),
            cpu_usage,
            memory: 0,
            user_id: None,
            state: String::new(),
            cmd: String::new(),
            exe_path: None,
            parent_pid: None,
            cmd_full: vec![],
            start_time: 0,
            run_time: 0,
            effective_user_id: None,
            group_id: None,
            cwd: None,
            thread_count: 0,
            open_files: 0,
            open_files_limit: 0,
            disk_total_read_bytes: 0,
            disk_total_write_bytes: 0,
            environ: vec![],
            session_id: None,
        }
    }

    fn ctx(host: &mut dyn HostState, caps: Vec<PluginCapability>) -> PluginContext<'_> {
        PluginContext::new(host, PathBuf::from("/tmp/xtop-plugin-test"), caps)
    }

    fn deny_err(err: PluginError) -> String {
        match err {
            PluginError::Recoverable(msg) => msg,
            other => panic!("expected Recoverable capability denial, got: {other:?}"),
        }
    }

    #[test]
    fn read_methods_are_denied_without_read_system_info() {
        let mut host = FakeHost::new(vec![]);
        let context = ctx(
            &mut host,
            vec![
                PluginCapability::KillProcesses,
                PluginCapability::ModifyConfig,
            ],
        );

        let snapshot = context.snapshot().unwrap_err();
        assert!(deny_err(snapshot).contains("ReadSystemInfo"));

        let system_info = context.system_info().unwrap_err();
        assert!(deny_err(system_info).contains("ReadSystemInfo"));

        let top = context.top_processes(3).unwrap_err();
        assert!(deny_err(top).contains("ReadSystemInfo"));
    }

    #[test]
    fn read_methods_succeed_with_read_system_info() {
        let mut host = FakeHost::new(vec![process(1, 1.0)]);
        let context = ctx(&mut host, vec![PluginCapability::ReadSystemInfo]);

        assert!(context.snapshot().is_ok());
        assert!(context.system_info().is_ok());
        assert!(context.top_processes(3).is_ok());
    }

    #[test]
    fn top_processes_sorts_by_cpu_desc_before_taking_n() {
        // Deliberately unsorted input: the contract must sort, not the host.
        let processes = vec![
            process(1, 10.0),
            process(2, 90.0),
            process(3, 55.5),
            process(4, 90.0),
            process(5, 1.0),
        ];
        let mut host = FakeHost::new(processes);
        let context = ctx(&mut host, vec![PluginCapability::ReadSystemInfo]);

        let top = context.top_processes(3).unwrap();
        let pids: Vec<u32> = top.iter().map(|p| p.pid).collect();
        // 90.0 twice keeps input order (stable sort): pid 2 before pid 4.
        assert_eq!(pids, vec![2, 4, 3]);
        assert!(top.windows(2).all(|w| w[0].cpu_usage >= w[1].cpu_usage));
    }

    #[test]
    fn top_processes_takes_at_most_n() {
        let processes = (1..=5).map(|i| process(i, f64::from(i))).collect();
        let mut host = FakeHost::new(processes);
        let context = ctx(&mut host, vec![PluginCapability::ReadSystemInfo]);

        assert_eq!(context.top_processes(2).unwrap().len(), 2);
        // More requested than available: everything is returned.
        assert_eq!(context.top_processes(99).unwrap().len(), 5);
    }
}
