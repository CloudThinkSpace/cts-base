pub mod csv;
pub mod geojson;
pub mod json;

use crate::response::{CtsResult, PageValue};
use serde_json::{json, Value};

pub trait PgRowConvert {
    fn convert(&self, data: CtsResult) -> Value;
}

pub fn page_to_value<F>(page: PageValue, func: F) -> Value
where
    F: Fn(CtsResult) -> Value,
{
    let current_page = page.current_page;
    let page_size = page.page_size;
    let pages = page.pages;
    let total = page.total;
    let list = page.list;
    // 数量数组
    let result = func(CtsResult::List(list));
    json!({
        "currentPage": current_page,
        "pageSize": page_size,
        "pages": pages,
        "total": total,
        "list": result
    })
}
