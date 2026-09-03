use crate::model::*;

/// Source of real-time system data.
///
/// Implemented by the kernel's sysinfo provider and by any plugin that
/// contributes extra metrics (merged via the kernel's composite provider).
pub trait SystemDataProvider: Send {
    fn refresh_all(&mut self);
    fn snapshot(&self) -> SystemSnapshot;
    fn disk_io(&self) -> Vec<DiskIOInfo> {
        vec![]
    }
    fn batteries(&self) -> Vec<BatteryInfo> {
        vec![]
    }
    fn gpu_info(&self) -> Vec<GpuInfo> {
        vec![]
    }
    fn docker_info(&self) -> Vec<DockerInfo> {
        vec![]
    }
    fn system_info(&self) -> SystemInfo {
        SystemInfo::default()
    }
    fn kill_process(&self, _pid: u32) -> bool {
        false
    }

    /// Downcast to `Any` for internal provider composition.
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    /// Add extra data providers (used by composite providers).
    /// Default no-op implementation for non-composite providers.
    fn add_extras(&mut self, _extras: Vec<Box<dyn SystemDataProvider>>) {}
}
