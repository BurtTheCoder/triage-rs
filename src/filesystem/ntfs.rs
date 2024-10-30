// src/filesystem/ntfs.rs
use super::{FileMetadata, FilesystemReader};
use crate::image::TskImage;
use anyhow::{Result, bail};
use std::path::Path;

pub struct NtfsReader<'a> {
    image: &'a TskImage,
    root_inode: u64,
}

impl<'a> NtfsReader<'a> {
    pub fn new(image: &'a TskImage) -> Result<Self> {
        // Get root directory inode
        let root_inode = unsafe {
            let fs_info = image.get_fs_info();
            if (*fs_info).ftype != ::TSK_FS_TYPE_ENUM::TSK_FS_TYPE_NTFS {
                bail!("Not an NTFS filesystem");
            }
            (*fs_info).root_inum
        };

        Ok(Self {
            image,
            root_inode,
        })
    }

    fn parse_mft_record(&self, inode: u64) -> Result<FileMetadata> {
        unsafe {
            let fs_info = self.image.get_fs_info();
            let file = tsk_fs_file_open_meta(fs_info, std::ptr::null_mut(), inode);
            if file.is_null() {
                bail!("Failed to open MFT record");
            }

            let meta = (*file).meta;
            let metadata = FileMetadata {
                created: (*meta).crtime,
                modified: (*meta).mtime,
                accessed: (*meta).atime,
                mft_modified: (*meta).ctime,
                size: (*meta).size,
                allocated: ((*meta).flags & ::TSK_FS_META_FLAG_ENUM::TSK_FS_META_FLAG_ALLOC as u32) != 0,
                is_directory: (*meta).type_ == ::TSK_FS_META_TYPE_ENUM::TSK_FS_META_TYPE_DIR as u32,
                attributes: (*meta).mode,
            };

            tsk_fs_file_close(file);
            Ok(metadata)
        }
    }
}

impl<'a> FilesystemReader for NtfsReader<'a> {
    fn read_file(&self, path: &str) -> Result<Vec<u8>> {
        self.image.read_file(&Path::new(path))
    }

    fn get_metadata(&self, path: &str) -> Result<FileMetadata> {
        let inode = self.path_to_inode(path)?;
        self.parse_mft_record(inode)
    }

    fn list_directory(&self, path: &str) -> Result<Vec<String>> {
        self.image.list_directory(&Path::new(path))
            .map(|entries| {
                entries.into_iter()
                    .map(|entry| entry.name)
                    .collect()
            })
    }
}