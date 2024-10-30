// src/registry/mod.rs
mod parser;
mod hive;
mod value;

pub use parser::RegistryParser;
pub use hive::{RegistryHive, RegistryKey};
pub use value::RegistryValue;