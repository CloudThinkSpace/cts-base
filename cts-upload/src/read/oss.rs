use crate::error::CtsUpLoadError;
use crate::read::CtsReader;
use crate::OssConfig;
use aliyun_oss_rust_sdk::oss::OSS;
use aliyun_oss_rust_sdk::request::RequestBuilder;

pub struct CtsOssReader {
    path: String,
    oss_config: OssConfig,
}

impl CtsOssReader {
    pub fn new(path: String, config: OssConfig) -> CtsOssReader {
        Self {
            path,
            oss_config: config,
        }
    }
}

impl CtsReader for CtsOssReader {
    async fn read(&self) -> Result<Vec<u8>, CtsUpLoadError> {
        let oss = OSS::new(
            &self.oss_config.key_id,
            &self.oss_config.key_secret,
            &self.oss_config.endpoint,
            &self.oss_config.bucket,
        );
        let build = RequestBuilder::new();
        oss.get_object(&self.path, build)
            .await
            .map_err(|err| CtsUpLoadError::ReadError(err.to_string()))
    }
}
