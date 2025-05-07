use crate::error::CtsUpLoadError;
use crate::utils::time_util::create_time_dir;
use crate::write::CtsWriter;
use crate::{get_ext, CtsFile, OssConfig};
use aliyun_oss_rust_sdk::oss::OSS;
use aliyun_oss_rust_sdk::request::RequestBuilder;
use axum::body::Bytes;
use uuid::Uuid;

pub struct CtsOssWriter<'a> {
    file_name: String,
    root_path: String,
    data: Bytes,
    config: &'a OssConfig,
}

impl<'a> CtsOssWriter<'a> {
    pub fn new(file_name: String, root_path: String, data: Bytes, config: &'a OssConfig) -> Self {
        Self {
            file_name,
            root_path,
            config,
            data,
        }
    }
}
impl CtsWriter for CtsOssWriter<'_> {
    async fn write(&self) -> Result<CtsFile, CtsUpLoadError> {
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
        let (name, ext) = get_ext(&self.file_name);
        // 文件路径
        let path = format!("{oss_path}/{uuid}.{}", &ext);
        // 上传文件
        oss.pub_object_from_buffer(&path, &self.data, build)
            .await
            .map_err(|err| CtsUpLoadError::WriteError(err.to_string()))?;

        Ok(CtsFile::new(self.file_name.to_string(), path, name, ext))
    }
}
