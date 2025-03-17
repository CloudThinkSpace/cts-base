pub mod json;
pub mod geojson;
pub mod csv;

use serde_json::Value;
use sqlx::postgres::PgRow;

pub trait PgRowConvert {
    fn convert(&self, row: Vec<PgRow>) -> Value;
}