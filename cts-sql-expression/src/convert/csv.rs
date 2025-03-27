use crate::convert::{page_to_value, PgRowConvert};
use crate::response::CtsResult;
use cts_pgrow::SerVecPgRow;
use serde_json::Value;

/// csv转换工具
/// > 将CstResult 转换成 数组[a,b,c,d]或者page
/// ```json
///     {
///         "currentPage": 1,
///         "pageSize": 10,
///         "pages": 100,
///        "total": 1000,
///         "list": [a,b,c,d,e...]
///      }
/// ```
pub struct CsvConvert;

impl PgRowConvert for CsvConvert {
    fn convert(&self, data: CtsResult) -> Value {
        handler_result(data)
    }
}

fn handler_result(data: CtsResult) -> Value {
    match data {
        CtsResult::Single(single) => {
            let vec_pg = SerVecPgRow::from(single);
            serde_json::to_value(&vec_pg).unwrap()
        }
        CtsResult::List(list) => {
            let mut result = Vec::new();
            for row in list.into_iter() {
                let vec_pg = SerVecPgRow::from(row);
                let value: Value = serde_json::to_value(&vec_pg).unwrap();
                result.push(value)
            }
            Value::Array(result)
        }
        CtsResult::Page(page) => page_to_value(page, handler_result),
    }
}
