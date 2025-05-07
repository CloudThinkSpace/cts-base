use serde::{Deserialize, Serialize};
pub mod error;
pub mod header;
#[cfg(feature = "multipart")]
pub mod multipart;
#[cfg(feature = "reader")]
pub mod read;
pub mod utils;
#[cfg(feature = "writer")]
pub mod write;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct CtsFile {
    pub path: String,
    pub filename: String,
    pub name: String,
    pub ext: String,
}

impl CtsFile {
    pub fn new(filename: String, path: String, name: String, ext: String) -> Self {
        Self {
            path,
            filename,
            name,
            ext,
        }
    }
}

pub fn get_ext(filename: &str) -> (String, String) {
    let index = filename.find('.').unwrap();
    let ext = &filename[index + 1..];
    let name = &filename[..index];
    (name.to_string(), ext.to_string())
}

#[cfg(test)]
mod tests {
    use crate::get_ext;

    #[test]
    fn test_get_ext() {
        let filename = "aaa.xls";
        let (a,b)  = get_ext(filename);
        println!("name:{},ext:{}", a, b);
    }
}
