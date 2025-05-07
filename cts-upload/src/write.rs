use std::future::Future;
use crate::{error::CtsUpLoadError, CtsFile};

pub mod file;
pub mod oss;

pub trait CtsWriter {
    fn write(&self) -> impl Future<Output = Result<CtsFile, CtsUpLoadError>> + Send;
}
