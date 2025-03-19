use aliyun_oss_rust_sdk::oss::OSS;
use aliyun_oss_rust_sdk::request::RequestBuilder;
use crate::error::CtsUpLoadError;
use crate::OssConfig;
use crate::read::CtsReader;

pub struct CtsOssReader(String, OssConfig);

impl CtsReader for CtsOssReader {
    async fn read(&self) -> Result<Vec<u8>, CtsUpLoadError> {
        let oss = OSS::new(
            &self.1.key_id,
            &self.1.key_secret,
            &self.1.endpoint,
            &self.1.bucket,
        );
        let build = RequestBuilder::new();
        oss.get_object(&self.0, build).await.map_err(|err|CtsUpLoadError::ReadError(err.to_string()))
    }
}