/// Error type for plugin operations.
#[derive(Debug)]
pub enum PluginError {
    /// A recoverable error (e.g. invalid params, resource busy)
    Recoverable(String),
    /// A fatal error (plugin should be disabled)
    Fatal(String),
    /// Action not understood by this plugin
    UnknownAction(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Recoverable(msg) => write!(f, "{msg}"),
            Self::Fatal(msg) => write!(f, "FATAL: {msg}"),
            Self::UnknownAction(action) => write!(f, "unknown action: {action}"),
        }
    }
}

impl std::error::Error for PluginError {}
