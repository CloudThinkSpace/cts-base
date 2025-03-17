use crate::error::CtsError;
use crate::error::CtsError::OrderError;
use crate::expression::parse::handler_name;
use crate::expression::{CtsValue, Single, SqlParse};

/// 排序字段
/// ```sql
///    [field]
///    或者
///    [field1,[field2,desc]]
/// ```
pub struct OrderByParse<'a>(pub &'a Option<Vec<CtsValue>>);

static ORDERS: [&str; 2] = ["asc", "desc"];

impl SqlParse for OrderByParse<'_> {
    fn parse(&self) -> Result<Option<String>, CtsError> {
        let order_param = self.0;
        match order_param {
            None => Ok(None),
            Some(data) => {
                let mut result = Vec::new();
                for datum in data.iter() {
                    match datum {
                        CtsValue::Single(Single::String(order)) => {
                            result.push(order.to_string());
                        }
                        CtsValue::Array(arr) => {
                            if arr.len() < 2 {
                                return Err(OrderError("排序参数错误".to_string()));
                            }
                            let name = handler_name(&arr[0])?;
                            let value = handler_name(&arr[1])?;
                            // 判断value 是否是asc或者desc
                            if !ORDERS.contains(&&*value.to_lowercase()) {
                                return Err(OrderError("排序参数错误".to_string()));
                            }
                            result.push(format!("{name} {value}"))
                        }
                        _ => {
                            return Err(OrderError("排序参数类型错误".to_string()));
                        }
                    }
                }
                Ok(Some(result.join(",")))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::parse::order::OrderByParse;
    use crate::expression::{CtsValue, Single, SqlParse};

    #[test]
    fn test() {
        let aa = Some(vec![
            CtsValue::Single(Single::String("aaa".to_string())),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("bbb".to_string())),
                CtsValue::Single(Single::String("asc".to_string())),
            ]),
        ]);
        let parse = OrderByParse(&aa).parse().unwrap();
        println!("{}", parse.unwrap())
    }
}
