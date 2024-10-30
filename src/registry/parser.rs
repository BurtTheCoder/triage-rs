// src/registry/parser.rs
use std::path::Path;
use std::collections::HashMap;
use std::io::{Read, Seek};
use anyhow::{Result, bail, Context};
use super::{RegistryHive, RegistryKey, RegistryValue};

#[derive(Debug)]
pub struct RegistryParser {
    /// Holds loaded registry hives indexed by their name (e.g., "SYSTEM", "SOFTWARE")
    hives: HashMap<String, RegistryHive>,
    /// Cache of recently accessed keys to improve performance
    key_cache: lru::LruCache<String, RegistryKey>,
}

impl RegistryParser {
    pub fn new() -> Self {
        Self {
            hives: HashMap::new(),
            key_cache: lru::LruCache::new(1000), // Cache last 1000 accessed keys
        }
    }

    /// Load a registry hive from a file
    pub fn load_hive<P: AsRef<Path>>(&mut self, path: P, name: &str) -> Result<()> {
        let hive = RegistryHive::from_file(path)?;
        self.hives.insert(name.to_string(), hive);
        Ok(())
    }

    /// Get a registry value by its full path
    pub fn get_value(&self, hive: &str, key_path: &str, value_name: &str) -> Result<RegistryValue> {
        let hive = self.hives.get(hive)
            .context(format!("Hive '{}' not found", hive))?;
        
        let key = self.get_key(hive, key_path)?;
        key.get_value(value_name)
    }

    /// Get a registry key by its path
    pub fn get_key(&self, hive: &str, path: &str) -> Result<RegistryKey> {
        let cache_key = format!("{}\\{}", hive, path);
        
        // Check cache first
        if let Some(key) = self.key_cache.get(&cache_key) {
            return Ok(key.clone());
        }

        let hive = self.hives.get(hive)
            .context(format!("Hive '{}' not found", hive))?;
        
        let key = hive.get_key(path)?;
        self.key_cache.put(cache_key, key.clone());
        
        Ok(key)
    }

    /// Enumerate all subkeys of a given key
    pub fn enumerate_subkeys(&self, hive: &str, path: &str) -> Result<Vec<RegistryKey>> {
        let key = self.get_key(hive, path)?;
        key.enumerate_subkeys()
    }

    /// Enumerate all values in a key
    pub fn enumerate_values(&self, hive: &str, path: &str) -> Result<HashMap<String, RegistryValue>> {
        let key = self.get_key(hive, path)?;
        key.enumerate_values()
    }

    /// Search for a value across all loaded hives
    pub fn search_value(&self, value_name: &str) -> Vec<(String, String, RegistryValue)> {
        let mut results = Vec::new();
        
        for (hive_name, hive) in &self.hives {
            if let Ok(matches) = hive.search_value(value_name) {
                for (key_path, value) in matches {
                    results.push((hive_name.clone(), key_path, value));
                }
            }
        }
        
        results
    }
}