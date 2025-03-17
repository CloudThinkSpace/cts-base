use crate::error::CtsError;
use crate::error::CtsError::FieldError;
use crate::expression::{CtsValue, Single, SqlParse};

/// # 字段解析器
/// > 字段解析器，主要是解析查询字段参数，字段参数只支持字符串一维数组或者二维数组
/// ```txt
/// ["field1","field2", "field3"]
/// // 或者
/// ["field1",["field2", "别名"],["field3", "别名2"]]
/// ```
pub struct FieldParse<'a>(pub &'a Option<Vec<CtsValue>>);

impl SqlParse for FieldParse<'_> {
    fn parse(&self) -> Result<Option<String>, CtsError> {
        let fields = self.0;
        match fields {
            None => Ok(None),
            Some(data) => {
                // 收集字段值
                let mut result = Vec::new();
                // 遍历字段
                for datum in data.iter() {
                    // 判断字段类型，如果是字符串直接收集，如果是数组继续解析
                    let field = handler_cts_value(datum)?;
                    result.push(field);
                }

                Ok(Some(result.join(",")))
            }
        }
    }
}
// 处理字段数组解析
pub fn handler_array(value: &CtsValue) -> Result<String, CtsError> {
    // 判断类型，如果是数组提示错误，是字符串直接收集
    match value {
        CtsValue::Single(field) => match field {
            Single::String(data) => Ok(data.to_string()),
            _ => Err(FieldError("查询字段类型只能是字符串".to_string())),
        },
        CtsValue::Array(_) => Err(FieldError("数据格式不对，请检查数据格式。".to_string())),
    }
}
// 处理field CtsValue数据
pub fn handler_cts_value(value: &CtsValue) -> Result<String, CtsError> {
    match value {
        CtsValue::Single(data) => {
            // 判断数据是否是字符串，如果是其他类型为错误
            match data {
                Single::String(value_str) => Ok(value_str.to_string()),
                _ => Err(FieldError("查询字段类型只能是字符串".to_string())),
            }
        }
        CtsValue::Array(data_array) => {
            // 判断数组长度
            match data_array.len() {
                0 => Err(FieldError("查询字段数组不能为空，请检查数据格式。".to_string())),
                1 => {
                    let param_0 = &data_array[0];
                    // 判断类型，如果是数组提示错误，是字符串直接收集
                    let field = handler_array(param_0)?;
                    Ok(field)
                }
                _ => {
                    // 别名数组
                    let param_0 = &data_array[0];
                    // 判断类型，如果是数组提示错误，是字符串直接收集
                    let field0 = handler_array(param_0)?;
                    let param_1 = &data_array[1];
                    // 判断类型，如果是数组提示错误，是字符串直接收集
                    let field1 = handler_array(param_1)?;
                    let field_alias = format!("{field0} as {field1}");
                    Ok(field_alias)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::parse::field::FieldParse;
    use crate::expression::{CtsValue, Single, SqlParse};

    #[test]
    fn test_field() {
        let field_param = Some(vec![
            CtsValue::Single(Single::String("aaa".to_string())),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("bb".to_string())),
                CtsValue::Single(Single::String("cc".to_string())),
            ]),
        ]);
        let field_parse = FieldParse(&field_param);
        let bb = field_parse.parse().unwrap();
        print!("{}", bb.unwrap())
    }
}
