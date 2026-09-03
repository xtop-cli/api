/// Error type for extension operations.
#[derive(Debug)]
pub enum ExtensionError {
    /// A recoverable error (e.g. invalid params, resource busy).
    Recoverable(String),
    /// A fatal error (extension should be disabled).
    Fatal(String),
    /// Server or action not understood by this extension.
    Unknown(String),
}

impl std::fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Recoverable(msg) => write!(f, "{msg}"),
            Self::Fatal(msg) => write!(f, "FATAL: {msg}"),
            Self::Unknown(msg) => write!(f, "unknown: {msg}"),
        }
    }
}

impl std::error::Error for ExtensionError {}
