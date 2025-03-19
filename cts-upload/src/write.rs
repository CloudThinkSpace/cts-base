use std::future::Future;
use crate::error::CtsUpLoadError;

pub mod file;
pub mod oss;

pub trait CtsWriter {
    fn write(&self) -> impl Future<Output = Result<(String, String), CtsUpLoadError>> + Send;
}