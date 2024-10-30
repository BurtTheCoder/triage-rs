// src/filesystem/mod.rs
mod ntfs;
mod ext;

pub use ntfs::NtfsReader;
pub use ext::ExtReader;

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub created: i64,
    pub modified: i64,
    pub accessed: i64,
    pub mft_modified: i64,
    pub size: u64,
    pub allocated: bool,
    pub is_directory: bool,
    pub attributes: u32,
}

pub trait FilesystemReader {
    fn read_file(&self, path: &str) -> anyhow::Result<Vec<u8>>;
    fn get_metadata(&self, path: &str) -> anyhow::Result<FileMetadata>;
    fn list_directory(&self, path: &str) -> anyhow::Result<Vec<String>>;
}