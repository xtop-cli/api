//! `xtop-extension-api` — shared extension protocol for xtop.
//!
//! Extensions are optional behaviors the kernel can host:
//!
//! - **Hooks** that touch config, theme, layout or rendering (designed as
//!   they are needed).
//! - **Servers**: long-running integrations (e.g. an MCP server) that the
//!   kernel starts on demand and that drive the app through an
//!   [`ExtensionHost`] view.
//!
//! Extensions never depend on the kernel; they only use this crate (and
//! `xtop-plugin-api` for the shared data model).

mod context;
mod error;
mod extension;
mod host;
mod manifest;

pub use context::ExtensionContext;
pub use error::ExtensionError;
pub use extension::Extension;
pub use host::ExtensionHost;
pub use manifest::ExtensionManifest;
