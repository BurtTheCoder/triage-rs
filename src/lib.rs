// src/lib.rs
use anyhow::Result;

pub mod image;
pub mod artifacts;
pub mod registry;
pub mod filesystem;
pub mod utils;

pub use image::TskImage;
pub use artifacts::collector::ForensicCollector;
pub use artifacts::types::{ArtifactInfo, SystemInfo};

/// Main entry point for forensic analysis
pub fn analyze_image(path: &std::path::Path) -> Result<SystemInfo> {
    let image = TskImage::new(path)?;
    let collector = ForensicCollector::new();
    collector.analyze(&image)
}