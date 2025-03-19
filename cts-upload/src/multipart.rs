mod oss;
mod file;

use crate::error::CtsUpLoadError;
use crate::multipart::oss::stream_to_oss;
use crate::OssConfig;
use axum::extract::Multipart;
use serde::{Deserialize, Serialize};
use crate::multipart::file::stream_to_file;

pub async  fn write_multipart_oss(mut multipart: Multipart, root_path:&str, oss_config: OssConfig) -> Result<Vec<CtsFile>, CtsUpLoadError> {
    let mut data_vec = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        // 处理文件
        if field.file_name().is_some() {
            let cts_file = stream_to_oss(field,root_path.to_string(),oss_config).await?;
            data_vec.push(cts_file);
        }
    }
    Ok(data_vec)
}

pub async  fn write_multipart_file(mut multipart: Multipart, root_path:&str)-> Result<Vec<CtsFile>, CtsUpLoadError> {
    let mut data_vec = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        // 处理文件
        if field.file_name().is_some() {
            let cts_file = stream_to_file(field,root_path.to_string()).await?;
            data_vec.push(cts_file);
        }
    }
    Ok(data_vec)
}



#[derive(Serialize, Deserialize, Debug)]
pub struct CtsFile {
    pub path: String,
    pub filename: String,
}