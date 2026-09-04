use serde::{Deserialize, Serialize};

/// Read-only view of the kernel's runtime configuration.
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub theme: String,
    pub layout: String,
    pub interval_ms: u64,
    pub hostname: String,
}

/// Current alert thresholds.
///
/// Serde round-trips under the plain field names `cpu_high`, `mem_high`,
/// `disk_high` (no rename attributes), so the kernel can persist this type
/// in its JSON config with the exact keys it serializes today.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cpu_high: f64,
    pub mem_high: f64,
    pub disk_high: f64,
}

/// The kernel-side surface a plugin may touch.
///
/// The kernel implements this trait for its live application state and hands
/// plugins a [`PluginContext`](crate::PluginContext) over it. Plugins never
/// depend on kernel types, only on this trait.
pub trait HostState {
    /// Full system snapshot with all available metrics.
    fn snapshot(&self) -> crate::SystemSnapshot;

    /// Current system info (hostname, OS, kernel).
    fn system_info(&self) -> crate::SystemInfo;

    /// Kill a process by PID. Returns true if the signal was sent.
    fn kill_process(&mut self, pid: u32) -> bool;

    /// Set alert thresholds for CPU, memory, and disk.
    fn set_alert_thresholds(&mut self, cpu: f64, mem: f64, disk: f64);

    /// Current alert thresholds.
    fn alerts(&self) -> AlertThresholds;

    /// Current runtime configuration (theme, layout, interval, hostname).
    fn config(&self) -> RuntimeConfig;

    /// Switch to a theme by name. Returns true if found.
    fn set_theme_by_name(&mut self, name: &str) -> bool;

    /// Switch to a layout by name. Returns true if found.
    fn set_layout_by_name(&mut self, name: &str) -> bool;

    /// Set the update interval in milliseconds.
    fn set_update_interval_ms(&mut self, ms: u64);
}

#[cfg(test)]
mod tests {
    use super::AlertThresholds;

    #[test]
    fn alert_thresholds_round_trip_under_snake_case_keys() {
        let alerts = AlertThresholds {
            cpu_high: 90.0,
            mem_high: 85.5,
            disk_high: 88.0,
        };

        let json = serde_json::to_string(&alerts).unwrap();
        assert_eq!(
            json,
            r#"{"cpu_high":90.0,"mem_high":85.5,"disk_high":88.0}"#
        );

        let back: AlertThresholds = serde_json::from_str(&json).unwrap();
        assert_eq!(back.cpu_high, alerts.cpu_high);
        assert_eq!(back.mem_high, alerts.mem_high);
        assert_eq!(back.disk_high, alerts.disk_high);
    }
}
