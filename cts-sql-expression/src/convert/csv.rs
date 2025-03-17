use cts_pgrow::SerVecPgRow;
use serde_json::Value;
use sqlx::postgres::PgRow;
use crate::convert::PgRowConvert;

pub struct CsvConvert;

impl PgRowConvert for CsvConvert {
    fn convert(&self, data: Vec<PgRow>) -> Value {
        let mut result = Vec::new();
        for row in data.into_iter() {
            let vec_pg = SerVecPgRow::from(row);
            let value = vec_pg.into();
            result.push(value)
        }
        Value::Array(result)
    }
}