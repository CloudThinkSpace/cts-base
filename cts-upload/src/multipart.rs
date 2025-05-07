mod file;
mod oss;

use crate::error::CtsUpLoadError;
use crate::multipart::file::stream_to_file;
use crate::multipart::oss::stream_to_oss;
use crate::{CtsFile, OssConfig};
use axum::extract::Multipart;
use std::collections::HashMap;

pub async fn write_multipart_oss(
    mut multipart: Multipart,
    root_path: &str,
    oss_config: OssConfig,
) -> Result<(HashMap<String, String>, Vec<CtsFile>), CtsUpLoadError> {
    // 收集存储的文件数组对象
    let mut data_vec = Vec::new();
    // 收集键值对数据，不包含文件字段
    let mut fields = HashMap::new();
    // 遍历字段
    while let Some(field) = multipart.next_field().await.unwrap() {
        // 获取key
        let name = field.name().unwrap().to_string();
        // 处理文件
        if field.file_name().is_some() {
            let cts_file = stream_to_oss(field, root_path.to_string(), &oss_config).await?;
            data_vec.push(cts_file);
        } else {
            // 获取值
            let text = field.text().await.unwrap();
            // 插入数据
            fields.insert(name, text);
        }
    }
    Ok((fields, data_vec))
}

pub async fn write_multipart_file(
    mut multipart: Multipart,
    root_path: &str,
) -> Result<(HashMap<String, String>, Vec<CtsFile>), CtsUpLoadError> {
    let mut data_vec = Vec::new();
    // 收集键值对数据，不包含文件字段
    let mut fields = HashMap::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        // 获取key
        let name = field.name().unwrap().to_string();
        // 处理文件
        if field.file_name().is_some() {
            let cts_file = stream_to_file(field, root_path.to_string()).await?;
            data_vec.push(cts_file);
        } else {
            // 获取值
            let text = field.text().await.unwrap();
            // 插入数据
            fields.insert(name, text);
        }
    }
    Ok((fields, data_vec))
}
