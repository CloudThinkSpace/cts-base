use crate::error::CtsUpLoadError;
use crate::multipart::CtsFile;
use crate::write::oss::CtsOssWriter;
use crate::write::CtsWriter;
use crate::OssConfig;
use axum::extract::multipart::Field;

pub async fn stream_to_oss(
    stream: Field<'_>,
    root_path: String,
    oss_config: OssConfig,
) -> Result<CtsFile, CtsUpLoadError> {
    let file_name = match &stream.file_name() {
        Some(data) => data.to_string(),
        None => "".to_string(),
    };
    // 数据
    let body = stream.bytes().await.unwrap();
    let writer = CtsOssWriter::new(file_name, root_path, body, oss_config);
    let (filename, path) = writer.write().await?;
    Ok(CtsFile { filename, path })
}
