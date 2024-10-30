// src/registry/hive.rs
use std::path::Path;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use anyhow::{Result, bail};

#[derive(Debug, Clone)]
pub struct RegistryHive {
    /// Raw hive data
    data: Vec<u8>,
    /// Offset to root key
    root_offset: u32,
    /// Hive header information
    header: HiveHeader,
}

#[derive(Debug, Clone)]
pub struct RegistryKey {
    /// Offset in the hive data
    offset: u32,
    /// Number of subkeys
    subkey_count: u32,
    /// Number of values
    value_count: u32,
    /// Name of the key
    name: String,
    /// Last write timestamp
    timestamp: i64,
}

#[derive(Debug, Clone)]
struct HiveHeader {
    signature: [u8; 4],
    sequence1: u32,
    sequence2: u32,
    timestamp: i64,
    major_version: u32,
    minor_version: u32,
    file_type: u32,
    format: u32,
    root_cell_offset: u32,
    length: u32,
}

impl RegistryHive {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let header = Self::parse_header(&data)?;
        
        Ok(Self {
            data,
            root_offset: header.root_cell_offset,
            header,
        })
    }

    fn parse_header(data: &[u8]) -> Result<HiveHeader> {
        if data.len() < 512 {
            bail!("Invalid hive file: too small");
        }

        let mut cursor = std::io::Cursor::new(data);
        
        let mut signature = [0u8; 4];
        cursor.read_exact(&mut signature)?;
        
        if &signature != b"regf" {
            bail!("Invalid hive signature");
        }

        Ok(HiveHeader {
            signature,
            sequence1: cursor.read_u32::<LittleEndian>()?,
            sequence2: cursor.read_u32::<LittleEndian>()?,
            timestamp: cursor.read_i64::<LittleEndian>()?,
            major_version: cursor.read_u32::<LittleEndian>()?,
            minor_version: cursor.read_u32::<LittleEndian>()?,
            file_type: cursor.read_u32::<LittleEndian>()?,
            format: cursor.read_u32::<LittleEndian>()?,
            root_cell_offset: cursor.read_u32::<LittleEndian>()?,
            length: cursor.read_u32::<LittleEndian>()?,
        })
    }

    pub fn get_key(&self, path: &str) -> Result<RegistryKey> {
        let mut current_key = self.get_root_key()?;
        
        if path.is_empty() {
            return Ok(current_key);
        }

        for component in path.split('\\') {
            let subkeys = current_key.enumerate_subkeys()?;
            current_key = subkeys.into_iter()
                .find(|k| k.name.eq_ignore_ascii_case(component))
                .context(format!("Subkey '{}' not found", component))?;
        }

        Ok(current_key)
    }

    pub fn get_root_key(&self) -> Result<RegistryKey> {
        self.parse_key(self.root_offset)
    }

    fn parse_key(&self, offset: u32) -> Result<RegistryKey> {
        // Implementation of registry key parsing
        // This would involve reading the binary structure of the registry key
        // and parsing its metadata, name, and other attributes
        unimplemented!("Registry key parsing not yet implemented")
    }

    pub fn search_value(&self, value_name: &str) -> Result<Vec<(String, RegistryValue)>> {
        let mut results = Vec::new();
        let root = self.get_root_key()?;
        self.search_value_recursive(&root, String::new(), value_name, &mut results)?;
        Ok(results)
    }

    fn search_value_recursive(
        &self,
        key: &RegistryKey,
        path: String,
        value_name: &str,
        results: &mut Vec<(String, RegistryValue)>
    ) -> Result<()> {
        // Check values in current key
        if let Ok(values) = key.enumerate_values() {
            if let Some(value) = values.get(value_name) {
                results.push((path.clone(), value.clone()));
            }
        }

        // Recurse into subkeys
        for subkey in key.enumerate_subkeys()? {
            let new_path = if path.is_empty() {
                subkey.name.clone()
            } else {
                format!("{}\\{}", path, subkey.name)
            };
            self.search_value_recursive(&subkey, new_path, value_name, results)?;
        }

        Ok(())
    }
}