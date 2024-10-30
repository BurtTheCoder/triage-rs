// src/image/mod.rs
mod tsk;
mod ewf;

pub use tsk::TskImage;
pub use ewf::EwfImage;

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    Raw,
    E01,
    AFF,
    VHD,
}

pub trait ForensicImage {
    fn get_size(&self) -> u64;
    fn get_sector_size(&self) -> u32;
    fn read_sector(&self, sector: u64, buffer: &mut [u8]) -> anyhow::Result<usize>;
}