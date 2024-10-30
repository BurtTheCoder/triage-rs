// src/image/tsk.rs
use std::path::{Path, PathBuf};
use std::ffi::{CString, CStr};
use std::ptr;
use anyhow::{Result, bail, Context};
use parking_lot::RwLock;
use std::sync::Arc;

pub struct TskImage {
    img_info: *mut ::tsk_img_info,
    fs_info: *mut ::tsk_fs_info,
    vs_info: Option<*mut ::tsk_vs_info>,
    path: PathBuf,
    sector_size: u32,
    size: u64,
    offset_table: Arc<RwLock<Vec<u64>>>,
}

impl TskImage {
    pub fn new(path: &Path) -> Result<Self> {
        unsafe {
            let path_str = CString::new(path.to_string_lossy().as_bytes())?;
            let img_info = if path.extension()
                .map_or(false, |ext| ext.to_string_lossy().to_lowercase().starts_with("e0"))
            {
                tsk_img_open_external(
                    CString::new("ewf")?.as_ptr(),
                    &path_str.as_ptr(),
                    1,
                    0
                )
            } else {
                tsk_img_open_utf8_sing(path_str.as_ptr())
            };

            if img_info.is_null() {
                bail!("Failed to open image: {}", path.display());
            }

            let vs_info = tsk_vs_open(img_info, 0, ::TSK_VS_TYPE_DETECT);
            let mut offset_table = Vec::new();

            if !vs_info.is_null() {
                for i in 0..(*vs_info).part_count {
                    let part = tsk_vs_part_get(vs_info, i);
                    if !part.is_null() {
                        offset_table.push((*part).start * (*vs_info).block_size);
                    }
                }
            }

            let offset = offset_table.first().copied().unwrap_or(0);
            let fs_info = tsk_fs_open_img(img_info, offset, ::TSK_FS_TYPE_DETECT);

            if fs_info.is_null() {
                if !vs_info.is_null() {
                    tsk_vs_close(vs_info);
                }
                tsk_img_close(img_info);
                bail!("Failed to open filesystem");
            }

            Ok(TskImage {
                img_info,
                fs_info,
                vs_info: if vs_info.is_null() { None } else { Some(vs_info) },
                path: path.to_owned(),
                sector_size: (*img_info).sector_size,
                size: (*img_info).size,
                offset_table: Arc::new(RwLock::new(offset_table)),
            })
        }
    }

    pub fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        unsafe {
            let path_str = CString::new(path.to_string_lossy().as_bytes())?;
            let file = tsk_fs_file_open(self.fs_info, ptr::null_mut(), path_str.as_ptr());
            
            if file.is_null() {
                bail!("Failed to open file: {}", path.display());
            }

            let meta = (*file).meta;
            if meta.is_null() {
                tsk_fs_file_close(file);
                bail!("File has no metadata");
            }

            let size = (*meta).size as usize;
            let mut buffer = vec![0u8; size];

            let read = tsk_fs_file_read(
                file,
                0,
                buffer.as_mut_ptr() as *mut i8,
                buffer.len() as u64,
                ::TSK_FS_FILE_READ_FLAG_ENUM::TSK_FS_FILE_READ_FLAG_NONE as i32,
            );

            tsk_fs_file_close(file);

            if read < 0 {
                bail!("Failed to read file");
            }

            buffer.truncate(read as usize);
            Ok(buffer)
        }
    }

    pub fn list_directory(&self, path: &Path) -> Result<Vec<DirEntry>> {
        unsafe {
            let path_str = CString::new(path.to_string_lossy().as_bytes())?;
            let dir = tsk_fs_file_open(self.fs_info, ptr::null_mut(), path_str.as_ptr());
            
            if dir.is_null() {
                bail!("Failed to open directory: {}", path.display());
            }

            let dir_handle = tsk_fs_dir_open_meta(self.fs_info, (*dir).meta.addr);
            if dir_handle.is_null() {
                tsk_fs_file_close(dir);
                bail!("Failed to open directory handle");
            }

            let mut entries = Vec::new();
            let mut idx = 0;

            loop {
                let name = tsk_fs_dir_get_name(dir_handle, idx);
                if name.is_null() {
                    break;
                }

                entries.push(DirEntry {
                    name: CStr::from_ptr((*name).name.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                    inode: (*name).meta_addr,
                    file_type: FileType::from_meta_type((*name).type_),
                });

                idx += 1;
            }

            tsk_fs_dir_close(dir_handle);
            tsk_fs_file_close(dir);

            Ok(entries)
        }
    }
}

impl Drop for TskImage {
    fn drop(&mut self) {
        unsafe {
            if let Some(vs_info) = self.vs_info {
                tsk_vs_close(vs_info);
            }
            if !self.fs_info.is_null() {
                tsk_fs_close(self.fs_info);
            }
            if !self.img_info.is_null() {
                tsk_img_close(self.img_info);
            }
        }
    }
}

#[derive(Debug)]
pub struct DirEntry {
    pub name: String,
    pub inode: u64,
    pub file_type: FileType,
}

#[derive(Debug)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    Special,
    Unknown,
}

impl FileType {
    fn from_meta_type(type_: u32) -> Self {
        match type_ {
            TSK_FS_META_TYPE_REG => FileType::Regular,
            TSK_FS_META_TYPE_DIR => FileType::Directory,
            TSK_FS_META_TYPE_LNK => FileType::Symlink,
            _ => FileType::Unknown,
        }
    }
}