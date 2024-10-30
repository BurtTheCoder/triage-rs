// src/artifacts/mod.rs
mod collector;
mod windows;
mod linux;
mod types;

pub use collector::ForensicCollector;
pub use windows::WindowsArtifactCollector;
pub use linux::LinuxArtifactCollector;
pub use types::*;