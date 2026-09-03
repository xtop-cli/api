use std::fmt::Debug;

use crate::context::ExtensionContext;
use crate::{ExtensionError, ExtensionManifest};

/// The core trait every extension must implement.
pub trait Extension: Debug + Send {
    /// Static metadata about this extension.
    fn manifest(&self) -> ExtensionManifest;

    /// Run one of the servers declared in the manifest until it ends.
    ///
    /// The kernel dispatches `xtop <server>` style commands to this method.
    fn run_server(
        &mut self,
        server_id: &str,
        _ctx: &mut ExtensionContext,
    ) -> Result<(), ExtensionError> {
        Err(ExtensionError::Unknown(format!(
            "server '{server_id}' is not provided by this extension"
        )))
    }
}
