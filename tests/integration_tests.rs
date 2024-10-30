// tests/integration_tests.rs
use forensic_triage::{ForensicCollector, TskImage, SystemInfo};
use std::path::PathBuf;
use anyhow::Result;

#[test]
fn test_image_loading() -> Result<()> {
    let image = TskImage::new(PathBuf::from("test_data/sample.E01"))?;
    assert!(image.get_size() > 0);
    Ok(())
}

#[test]
fn test_windows_detection() -> Result<()> {
    let image = TskImage::new(PathBuf::from("test_data/windows.E01"))?;
    let collector = ForensicCollector::new();
    let info = collector.analyze(&image)?;
    
    assert!(matches!(info.os_type, OsType::Windows));
    assert!(info.hostname.len() > 0);
    Ok(())
}

#[test]
fn test_linux_detection() -> Result<()> {
    let image = TskImage::new(PathBuf::from("test_data/linux.E01"))?;
    let collector = ForensicCollector::new();
    let info = collector.analyze(&image)?;
    
    assert!(matches!(info.os_type, OsType::Linux));
    assert!(info.hostname.len() > 0);
    Ok(())
}

#[test]
fn test_artifact_collection() -> Result<()> {
    let image = TskImage::new(PathBuf::from("test_data/sample.E01"))?;
    let collector = ForensicCollector::new();
    let info = collector.analyze(&image)?;
    
    // Verify common artifacts are found
    assert!(info.artifacts.iter().any(|a| a.path.to_string_lossy().contains("SYSTEM")));
    assert!(info.artifacts.iter().any(|a| a.path.to_string_lossy().contains("NTUSER.DAT")));
    
    Ok(())
}

#[test]
fn test_registry_parsing() -> Result<()> {
    let image = TskImage::new(PathBuf::from("test_data/windows.E01"))?;
    let collector = ForensicCollector::new();
    let info = collector.analyze(&image)?;
    
    // Verify registry data is parsed
    assert!(info.hostname.len() > 0);
    assert!(info.os_version.is_some());
    
    Ok(())
}

#[test]
fn test_user_enumeration() -> Result<()> {
    let image = TskImage::new(PathBuf::from("test_data/windows.E01"))?;
    let collector = ForensicCollector::new();
    let info = collector.analyze(&image)?;
    
    assert!(!info.users.is_empty());
    assert!(info.users.iter().any(|u| u.username == "Administrator"));
    
    Ok(())
}