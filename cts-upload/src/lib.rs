use serde::Deserialize;

pub mod read;
pub mod write;
pub mod error;
pub mod multipart;
pub mod utils;

pub enum ModeType {
    File,
    OSS,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OssConfig {
    pub key_id: String,
    pub key_secret: String,
    pub endpoint: String,
    pub bucket: String,
}

pub fn get_ext(filename: &str) -> String {
    let index = filename.find('.').unwrap();
    let ext = &filename[index + 1..];
    ext.to_string()
}