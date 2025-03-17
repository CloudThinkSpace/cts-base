use serde_json::Value;
use sqlx::postgres::{PgRow, PgValue};
use crate::error::CtsError;

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