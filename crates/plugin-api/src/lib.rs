//! `xtop-plugin-api` — shared plugin protocol for xtop.
//!
//! Contract types used by both sides:
//!
//! - **Host** (the `xtop` kernel): discovers, registers and drives plugins.
//! - **Plugins** (the `xtop-cli/plugins` repo): implement [`Plugin`] against a
//!   [`HostState`] provided by the kernel.
//!
//! This crate never depends on the kernel, so every repo in the ecosystem can
//! consume it standalone.

pub mod model;

mod color;

mod capability;
mod context;
mod error;
mod host;
mod manifest;
mod plugin;
mod provider;
mod widget;

pub use capability::PluginCapability;
pub use color::hex_to_rgb;
pub use context::PluginContext;
pub use error::PluginError;
pub use host::{AlertThresholds, HostState, RuntimeConfig};
pub use manifest::PluginManifest;
pub use model::{
    BatteryInfo, CpuInfo, DiskIOInfo, DiskInfo, DockerInfo, GpuInfo, LoadAvg, MemoryInfo,
    NetworkInfo, ProcessInfo, SwapInfo, SystemInfo, SystemSnapshot,
};
pub use plugin::Plugin;
pub use provider::SystemDataProvider;
pub use widget::WidgetRegistration;
