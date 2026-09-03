/// Unique identifier for a plugin capability.
/// Used for permission checking and manifest declaration.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum PluginCapability {
    /// Read system metrics (CPU, memory, network, disks, processes)
    ReadSystemInfo,
    /// Terminate processes
    KillProcesses,
    /// Modify configuration (themes, layouts, alerts, interval)
    ModifyConfig,
    /// Register custom widgets in the TUI
    RenderWidgets,
    /// Anything not covered above
    Custom(String),
}
