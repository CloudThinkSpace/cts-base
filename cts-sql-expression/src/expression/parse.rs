use crate::error::CtsError;
use crate::error::CtsError::ParamError;
use crate::expression::{CtsValue, Single};

pub mod filter;
pub mod group;
pub mod field;
pub mod aggregate;
pub mod order;
pub mod page;

fn handler_name(data: &CtsValue) -> Result<String, CtsError> {
    match data {
        CtsValue::Single(Single::String(field)) => {
            Ok(field.to_string())
        }
        _ => {
            Err(ParamError("参数错误".to_string()))
        }
    }
}


