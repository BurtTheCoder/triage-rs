// src/main.rs
use anyhow::Result;
use clap::Parser;
use forensic_triage::{analyze_image, SystemInfo};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Path to forensic image
    #[clap(parse(from_os_str))]
    image: PathBuf,

    /// Output directory for artifacts
    #[clap(short, long, parse(from_os_str), default_value = "output")]
    output: PathBuf,

    /// Number of threads to use
    #[clap(short, long)]
    threads: Option<usize>,

    /// Enable verbose logging
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    if args.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    // Set number of threads
    if let Some(threads) = args.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()?;
    }

    // Create output directory
    std::fs::create_dir_all(&args.output)?;

    // Analyze image
    log::info!("Analyzing image: {}", args.image.display());
    let system_info = analyze_image(&args.image)?;

    // Write results
    let output_file = args.output.join("triage_results.json");
    std::fs::write(
        &output_file,
        serde_json::to_string_pretty(&system_info)?,
    )?;

    log::info!("Analysis complete. Results written to: {}", output_file.display());
    Ok(())
}