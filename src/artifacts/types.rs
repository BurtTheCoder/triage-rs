// src/artifacts/types.rs
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub ip_address: Option<String>,
    pub domain: Option<String>,
    pub os_type: OsType,
    pub os_version: Option<String>,
    pub install_date: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub users: Vec<UserInfo>,
    pub artifacts: Vec<ArtifactInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OsType {
    Windows,
    Linux,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub sid: Option<String>,
    pub profile_path: PathBuf,
    pub last_login: Option<DateTime<Utc>>,
    pub account_created: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtifactInfo {
    pub path: PathBuf,
    pub size: u64,
    pub hash: Option<String>,
    pub metadata: FileMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub accessed: DateTime<Utc>,
    pub is_directory: bool,
    pub permissions: u32,
}