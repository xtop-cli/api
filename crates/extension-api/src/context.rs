use crate::{ExtensionError, ExtensionHost};

/// Context passed to an extension while it runs.
pub struct ExtensionContext<'a> {
    host: &'a mut dyn ExtensionHost,
}

impl<'a> ExtensionContext<'a> {
    /// Create a new context over a host. Used by the kernel.
    pub fn new(host: &'a mut dyn ExtensionHost) -> Self {
        Self { host }
    }

    /// Advance the monitoring tick.
    pub fn tick(&mut self) {
        self.host.tick();
    }

    /// Execute a named action on a hosted plugin.
    pub fn execute_plugin(
        &mut self,
        plugin_id: &str,
        action: &str,
        params: &str,
    ) -> Result<String, ExtensionError> {
        self.host.execute_plugin(plugin_id, action, params)
    }
}
