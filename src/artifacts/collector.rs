// src/artifacts/collector.rs
use crate::image::TskImage;
use crate::utils::progress::ProgressTracker;
use anyhow::Result;
use rayon::prelude::*;
use std::sync::Arc;

pub struct ForensicCollector {
    progress: Arc<ProgressTracker>,
    max_file_size: Option<u64>,
    calculate_hashes: bool,
}

impl ForensicCollector {
    pub fn new() -> Self {
        Self {
            progress: Arc::new(ProgressTracker::new()),
            max_file_size: None,
            calculate_hashes: true,
        }
    }

    pub fn analyze(&self, image: &TskImage) -> Result<SystemInfo> {
        // Try Windows analysis first
        if let Ok(info) = WindowsArtifactCollector::new(image, Arc::clone(&self.progress))
            .collect() {
            return Ok(info);
        }

        // Fall back to Linux analysis
        if let Ok(info) = LinuxArtifactCollector::new(image, Arc::clone(&self.progress))
            .collect() {
            return Ok(info);
        }

        Ok(SystemInfo {
            hostname: String::from("unknown"),
            ip_address: None,
            domain: None,
            os_type: OsType::Unknown,
            os_version: None,
            install_date: None,
            timezone: None,
            users: Vec::new(),
            artifacts: Vec::new(),
        })
    }
}