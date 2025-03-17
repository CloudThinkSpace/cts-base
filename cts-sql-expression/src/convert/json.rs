use serde_json::Value;
use sqlx::postgres::PgRow;
use cts_pgrow::SerMapPgRow;
use crate::convert::PgRowConvert;

pub struct JsonConvert;
impl PgRowConvert for JsonConvert {
    fn convert(&self, data: Vec<PgRow>) -> Value {
        let mut result = Vec::new();
        for row in data.into_iter() {
            let row_map = SerMapPgRow::from(row);
            let value = row_map.into();
            result.push(value)
        }
        Value::Array(result)
    }
}