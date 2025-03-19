use crate::error::CtsUpLoadError;
use crate::multipart::CtsFile;
use crate::write::file::CtsFileWriter;
use crate::write::CtsWriter;
use axum::extract::multipart::Field;

pub async fn stream_to_file(
    stream: Field<'_>,
    root_path: String,
) -> Result<CtsFile, CtsUpLoadError> {
    let file_name = match &stream.file_name() {
        Some(data) => data.to_string(),
        None => "".to_string(),
    };
    // 数据
    let body = stream.bytes().await.unwrap();
    let writer = CtsFileWriter::new(file_name, root_path, body);
    let (filename, path) = writer.write().await?;
    Ok(CtsFile { filename, path })
}