use std::future::Future;
use crate::error::CtsUpLoadError;

pub mod file;
pub mod oss;

pub trait CtsReader {
    fn read(&self) -> impl Future<Output = Result<Vec<u8>, CtsUpLoadError>> + Send;
}