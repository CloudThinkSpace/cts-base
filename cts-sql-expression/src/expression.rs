pub mod parse;
pub mod sql;

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use crate::error::CtsError;
use crate::error::CtsError::ParamError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CtsParam {
    pub filter: Option<Vec<CtsValue>>,
    pub group_by: Option<Vec<String>>,
    pub out_fields: Option<Vec<CtsValue>>,
    pub aggregate: Option<Vec<CtsValue>>,
    pub return_geometry: Option<bool>,
    pub order: Option<Vec<CtsValue>>,
    pub page: Option<PageParam>,
    pub geo_format: Option<GeometryFormat>,
    pub format: Option<CtsFormat>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CtsFormat {
    Json,
    GeoJson,
    CSV,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GeometryFormat {
    GeoJson,
    WKT,
    Byte,
    Text,
    WKB,
}

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
    pub data_type: String,
    pub is_nullable: String,
    pub column_default: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageParam {
    pub page: i32,
    pub page_size: i32,
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


#[derive(Debug)]
pub enum CtsResult {
    List(Vec<PgRow>),
    Page(PageValue),
}

#[derive(Debug)]
pub struct PageValue {
    pub current_page: i32,
    pub page_size: i32,
    pub pages: i64,
    pub total: i64,
    pub list: Vec<PgRow>,
}
