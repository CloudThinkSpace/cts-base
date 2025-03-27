use crate::convert::{page_to_value, PgRowConvert};
use crate::response::CtsResult;
use cts_pgrow::SerMapPgRow;
use serde_json::Value;

/// json转换工具
/// > 将CstResult 转换成json或者page
/// - json数组
/// ```json
///     {
///         "currentPage": 1,
///         "pageSize": 10,
///       "pages": 100,
///         "total": 1000,
///         "list": [a,b,c,d,e...]
///      }
/// ```
/// - page json
/// ```json
///     {
///         "currentPage": 1,
///         "pageSize": 10,
///         "pages": 100,
///         "total": 1000,
///         "list": [
///             {
///                 "a":1,
///                 "b":2,
///                 "c":3,
///             }
///         ]
///      }
/// ```
pub struct JsonConvert;
impl PgRowConvert for JsonConvert {
    fn convert(&self, data: CtsResult) -> Value {
        handler_result(data)
    }
}

fn handler_result(data: CtsResult) -> Value {
    match data {
        CtsResult::Single(single) => {
            let row_map = SerMapPgRow::from(single);
            row_map.into()
        }
        CtsResult::List(list) => {
            let mut result = Vec::new();
            for row in list.into_iter() {
                let row_map = SerMapPgRow::from(row);
                let value = row_map.into();
                result.push(value)
            }
            Value::Array(result)
        }
        CtsResult::Page(page) => page_to_value(page, handler_result),
    }
}
