/// Static metadata about an extension.
#[derive(Clone, Debug)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    /// Server ids this extension can run (e.g. `"mcp"`).
    pub servers: Vec<String>,
}
