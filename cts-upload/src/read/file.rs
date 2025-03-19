use std::fs::read;
use crate::error::CtsUpLoadError;
use crate::read::CtsReader;

pub struct CtsFileReader(pub String);
impl CtsReader for CtsFileReader {
    async fn read(&self) -> Result<Vec<u8>, CtsUpLoadError> {
        let data = read(&self.0).map_err(|err| CtsUpLoadError::ReadError(err.to_string()))?;
        Ok(data)
    }
}