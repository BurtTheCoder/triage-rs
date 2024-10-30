// examples/basic_triage.rs

// Import necessary dependencies:
// - anyhow::Result for error handling
// - ForensicCollector for artifact collection
// - TskImage for forensic image handling
// - PathBuf for cross-platform path manipulation
use anyhow::Result;
use forensic_triage::{ForensicCollector, TskImage};
use std::path::PathBuf;

fn main() -> Result<()> {
    // STEP 1: Initialize the Forensic Collector
    // This creates a new collector with default settings:
    // - Default thread count (number of CPU cores)
    // - Default artifact patterns
    // - Default output directory
    let collector = ForensicCollector::new();
    
    // STEP 2: Prepare the Image Path
    // Create a path to the forensic image
    // PathBuf provides cross-platform path handling
    // You can also use different formats:
    // let image_path = PathBuf::from("path/to/disk.raw");
    // let image_path = PathBuf::from("path/to/memory.dump");
    let image_path = PathBuf::from("test_data/sample.E01");
    
    // STEP 3: Open the Forensic Image
    // TskImage::new() will:
    // - Detect the image format (E01, RAW, etc.)
    // - Initialize The Sleuth Kit
    // - Set up file system analysis
    // The ? operator propagates any errors that occur
    let image = TskImage::new(&image_path)?;
    
    // STEP 4: Analyze the Image
    // This is where the main analysis happens:
    // 1. Detect operating system
    // 2. Find and extract artifacts
    // 3. Parse registry (if Windows)
    // 4. Collect user information
    // 5. Generate system information
    let system_info = collector.analyze(&image)?;
    
    // STEP 5: Display Results
    
    // Print basic system information
    println!("Hostname: {}", system_info.hostname);
    
    // Print OS version if available
    // Using if let Some() to safely handle the Option<String>
    if let Some(os) = system_info.os_version.as_ref() {
        println!("OS Version: {}", os);
    }
    
    // Print user information
    println!("\nUsers found:");
    for user in &system_info.users {
        // For each user, print:
        // - Username
        // - Profile path (using display() for proper path formatting)
        println!("- {} ({})", user.username, user.profile_path.display());
    }
    
    // Print artifact count
    println!("\nArtifacts collected: {}", system_info.artifacts.len());
    
    // Return Ok if everything succeeded
    Ok(())
}

// You could extend this example with more detailed output:
fn print_detailed_results(system_info: &SystemInfo) {
    println!("=== System Information ===");
    println!("Hostname: {}", system_info.hostname);
    println!("OS Type: {:?}", system_info.os_type);
    
    if let Some(ip) = &system_info.ip_address {
        println!("IP Address: {}", ip);
    }
    
    if let Some(domain) = &system_info.domain {
        println!("Domain: {}", domain);
    }
    
    println!("\n=== Users ===");
    for user in &system_info.users {
        println!("Username: {}", user.username);
        println!("Profile Path: {}", user.profile_path.display());
        if let Some(last_login) = user.last_login {
            println!("Last Login: {}", last_login);
        }
        println!("---");
    }
    
    println!("\n=== Artifacts ===");
    for artifact in &system_info.artifacts {
        println!("Path: {}", artifact.path.display());
        println!("Size: {} bytes", artifact.size);
        if let Some(hash) = &artifact.hash {
            println!("Hash: {}", hash);
        }
        println!("Created: {}", artifact.metadata.created);
        println!("Modified: {}", artifact.metadata.modified);
        println!("---");
    }
}

// You could also add error handling:
fn analyze_with_error_handling(image_path: PathBuf) -> Result<()> {
    let collector = ForensicCollector::new();
    
    // Detailed error handling
    let image = TskImage::new(&image_path).map_err(|e| {
        eprintln!("Failed to open image: {}", e);
        e
    })?;
    
    let system_info = collector.analyze(&image).map_err(|e| {
        eprintln!("Analysis failed: {}", e);
        e
    })?;
    
    // Save results to file
    let output_path = PathBuf::from("analysis_results.json");
    serde_json::to_writer_pretty(
        std::fs::File::create(output_path)?,
        &system_info
    )?;
    
    Ok(())
}