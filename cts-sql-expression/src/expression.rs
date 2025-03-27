pub mod parse;
pub mod sql;
mod query_builder;

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use crate::error::CtsError;
use crate::error::CtsError::ParamError;

pub static GEOMETRY: &str = "geom";



#[derive(Debug, Serialize)]
pub enum CtsValue {
    Single(Single),
    Array(Vec<CtsValue>),
}

#[derive(Debug, Serialize)]
pub enum Single {
    String(String),
    Integer(i64),
    Double(f64),
    Bool(bool),
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Course {
    pub column_name: String,
    pub udt_name: String,
}

impl<'de> Deserialize<'de> for CtsValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        handler_value(value).map_err(|_err| {
            Error::custom("类型解析错误")
        })
    }
}

fn handler_value(data: Value) -> Result<CtsValue, CtsError> {
    let value = match data {
        Value::String(str) => CtsValue::Single(Single::String(str)),
        Value::Number(num) => {
            if num.is_f64() {
                CtsValue::Single(Single::Double(num.as_f64().unwrap()))
            } else {
                CtsValue::Single(Single::Integer(num.as_i64().unwrap()))
            }
        }
        Value::Bool(bool) => CtsValue::Single(Single::Bool(bool)),
        Value::Array(data) => {
            let mut result = Vec::new();
            for datum in data {
                let sub_data = handler_value(datum)?;
                result.push(sub_data);
            }
            CtsValue::Array(result)
        }
        _ => return Err(ParamError("类型错误".to_string())),
    };
    Ok(value)
}

pub trait SqlParse
{
    fn parse(&self) -> Result<Option<String>, CtsError>;
}



