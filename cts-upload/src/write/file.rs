use crate::error::CtsUpLoadError;
use crate::utils::time_util::create_time_dir;
use crate::write::CtsWriter;
use crate::{get_ext, CtsFile};
use axum::body::Bytes;
use std::fs::create_dir_all;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub struct CtsFileWriter {
    file_name: String,
    root_path: String,
    data: Bytes,
}

impl CtsFileWriter {
    pub fn new(file_name: String, root_path: String, data: Bytes) -> Self {
        Self {
            file_name,
            root_path,
            data,
        }
    }
}
impl CtsWriter for CtsFileWriter {
    async fn write(&self) -> Result<CtsFile, CtsUpLoadError> {
        // 创建日期路径字符串
        let file_path = create_time_dir(&self.root_path);
        // 创建日期目录
        create_dir_all(&file_path).map_err(|err| CtsUpLoadError::WriteError(err.to_string()))?;
        let uuid = Uuid::new_v4().to_string();
        // 文件名和扩展名
        let (name, ext) = get_ext(&self.file_name);
        // 文件路径
        let file_name_path = format!("{file_path}/{uuid}.{}", &ext);
        // 打开文件（如果文件不存在则创建）
        let mut file = File::create(&file_name_path)
            .await
            .map_err(|err| CtsUpLoadError::WriteError(err.to_string()))?;
        // 写入文件
        file.write_all(&self.data)
            .await
            .map_err(|err| CtsUpLoadError::WriteError(err.to_string()))?;
        // 确保所有数据都写入磁盘
        file.flush()
            .await
            .map_err(|err| CtsUpLoadError::WriteError(err.to_string()))?;

        // 组织数据
        Ok(CtsFile::new(
            self.file_name.to_string(),
            file_name_path,
            name,
            ext,
        ))
    }
}
