// examples/custom_artifacts.rs
use anyhow::Result;
use forensic_triage::{ForensicCollector, TskImage};
use std::path::PathBuf;

struct CustomArtifactCollector {
    patterns: Vec<String>,
}

impl CustomArtifactCollector {
    fn new() -> Self {
        Self {
            patterns: vec![
                String::from("**/*.pst"),         // Outlook data files
                String::from("**/*.ost"),         // Offline Outlook data
                String::from("**/*.msg"),         // Email messages
                String::from("**/*.docx"),        // Word documents
                String::from("**/*.xlsx"),        // Excel spreadsheets
                String::from("**/*.pdf"),         // PDF documents
            ],
        }
    }

    fn collect(&self, image: &TskImage) -> Result<Vec<PathBuf>> {
        let mut artifacts = Vec::new();
        
        // Implement custom collection logic here
        // This is just a placeholder
        for pattern in &self.patterns {
            println!("Collecting artifacts matching: {}", pattern);
        }
        
        Ok(artifacts)
    }
}

fn main() -> Result<()> {
    let image_path = PathBuf::from("test_data/sample.E01");
    let image = TskImage::new(&image_path)?;
    
    let custom_collector = CustomArtifactCollector::new();
    let artifacts = custom_collector.collect(&image)?;
    
    println!("Found {} custom artifacts", artifacts.len());
    
    Ok(())
}