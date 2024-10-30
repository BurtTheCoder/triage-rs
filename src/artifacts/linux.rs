// src/artifacts/linux.rs
pub struct LinuxArtifactCollector<'a> {
    image: &'a TskImage,
    progress: Arc<ProgressTracker>,
}

impl<'a> LinuxArtifactCollector<'a> {
    pub fn new(image: &'a TskImage, progress: Arc<ProgressTracker>) -> Self {
        Self { image, progress }
    }

    pub fn collect(&self) -> Result<SystemInfo> {
        // Linux analysis implementation
        unimplemented!("Linux analysis not yet implemented");
    }
}