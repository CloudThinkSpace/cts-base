use crate::convert::csv::CsvConvert;
use crate::convert::geojson::GeoJsonConvert;
use crate::convert::json::JsonConvert;
use crate::convert::PgRowConvert;
use crate::request::CtsFormat;
use serde_json::{json, Value};
use sqlx::postgres::PgRow;

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

impl CtsResult {
    pub fn to_value(self, format: CtsFormat, geometry: Option<String>) -> Value {
        // 配置转换器
        let row_convert: Box<dyn PgRowConvert> = match format {
            // 匹配 类型是GeoJson 并且空间字段不为空
            CtsFormat::GeoJson => Box::new(GeoJsonConvert(
                geometry.unwrap_or_else(|| "geom".to_string())
            )),
            CtsFormat::CSV => Box::new(CsvConvert),
            _ => Box::new(JsonConvert),
        };

        match self {
            CtsResult::List(list) => row_convert.convert(list),
            CtsResult::Page(page) => {
                let current_page = page.current_page;
                let page_size = page.page_size;
                let pages = page.pages;
                let total = page.total;
                let list = row_convert.convert(page.list);

                json!({
                    "currentPage": current_page,
                    "pageSize": page_size,
                    "pages": pages,
                    "total": total,
                    "list": list
                })
            }
        }
    }

    pub fn to_json(self) -> Value {
        self.to_value(CtsFormat::Json, None)
    }
}
