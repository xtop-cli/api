use crate::PluginCapability;

/// Static metadata about a plugin.
/// Returned by [`Plugin::manifest`](crate::Plugin::manifest).
#[derive(Clone, Debug)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub capabilities: Vec<PluginCapability>,
}
