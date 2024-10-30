// src/registry/value.rs
#[derive(Debug, Clone)]
pub enum RegistryValue {
    String(String),
    ExpandString(String),
    Binary(Vec<u8>),
    Dword(u32),
    Qword(u64),
    MultiString(Vec<String>),
    None,
}

impl RegistryValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            RegistryValue::String(s) | RegistryValue::ExpandString(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<u64> {
        match self {
            RegistryValue::Dword(v) => Some(*v as u64),
            RegistryValue::Qword(v) => Some(*v),
            _ => None,
        }
    }
}