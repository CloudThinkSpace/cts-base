use crate::error::CtsUpLoadError;
use crate::write::CtsWriter;
use crate::{get_ext, OssConfig};
use aliyun_oss_rust_sdk::oss::OSS;
use aliyun_oss_rust_sdk::request::RequestBuilder;
use axum::body::Bytes;
use uuid::Uuid;
use crate::utils::time_util::create_time_dir;

pub struct CtsOssWriter {
    file_name: String,
    root_path: String,
    data: Bytes,
    config: OssConfig,
}

impl CtsOssWriter {
    pub fn new(file_name: String, root_path: String, data: Bytes, config: OssConfig) -> Self {
        Self {
            file_name,
            root_path,
            config,
            data,
        }
    }
}
impl CtsWriter for CtsOssWriter {
    async fn write(&self) -> Result<(String, String), CtsUpLoadError> {
        let oss = OSS::new(
            &self.config.key_id,
            &self.config.key_secret,
            &self.config.endpoint,
            &self.config.bucket,
        );
        let build = RequestBuilder::new();
        // 根路径
        let oss_path = create_time_dir(&self.root_path);
        let uuid = Uuid::new_v4().to_string();
        // 扩展名
        let exp = get_ext(&self.file_name);
        // 文件路径
        let path = format!("{oss_path}/{uuid}.{exp}");
        // 上传文件
        oss.pub_object_from_buffer(&path, &self.data, build)
            .await
            .map_err(|err| CtsUpLoadError::WriteError(err.to_string()))?;

        Ok((self.file_name.to_string(), path))
    }
}
