use crate::ExtensionError;

/// The kernel-side surface an extension may drive.
///
/// The kernel implements this for its live application state and hands
/// extensions an [`ExtensionContext`](crate::ExtensionContext) over it.
/// Plugins remain the unit of domain behavior: extensions act through them
/// with [`execute_plugin`](ExtensionHost::execute_plugin).
pub trait ExtensionHost {
    /// Advance the monitoring tick (also ticks hosted plugins).
    fn tick(&mut self);

    /// Execute a named action on a hosted plugin (`plugin_id`).
    ///
    /// Returns the plugin's JSON-ish string response.
    fn execute_plugin(
        &mut self,
        plugin_id: &str,
        action: &str,
        params: &str,
    ) -> Result<String, ExtensionError>;
}
