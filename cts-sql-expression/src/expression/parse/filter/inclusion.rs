use crate::error::CtsError;
use crate::error::CtsError::FilterError;
use crate::expression::parse::handler_name;
use crate::expression::{CtsValue, Single, SqlParse};

pub struct InParse<'a>(pub &'a Vec<CtsValue>);
pub struct LikeParse<'a>(pub &'a Vec<CtsValue>);
pub struct BetweenParse<'a>(pub &'a Vec<CtsValue>);

impl SqlParse for InParse<'_> {
    fn parse(&self) -> Result<Option<String>, CtsError> {
        let data = self.0;
        if data.len() < 3 {
            return Err(FilterError("参数错误，参数长度不够".to_string()));
        }
        // 操作符
        let first = &data[0];
        // 解析操作符
        let ope = handler_name(first)?;
        // 字段
        let second = &data[1];
        let field = handler_name(second)?;
        // 值
        let third = &data[2];
        let value = handler_in_value(third)?;

        if value.is_empty() {
            Ok(Some("1 != 1".to_string()))
        } else {
            Ok(Some(format!("{field} {ope} ({value})")))
        }
    }
}

impl SqlParse for LikeParse<'_> {
    fn parse(&self) -> Result<Option<String>, CtsError> {
        let data = self.0;
        if data.len() < 3 {
            return Err(FilterError("LIKE参数错误，参数长度不够".to_string()));
        }
        // 操作符
        let first = &data[0];
        // 解析操作符
        let ope = handler_name(first)?;
        // 字段
        let second = &data[1];
        let field = handler_name(second)?;
        // 值
        let third = &data[2];
        let value = handler_like_value(third)?;

        Ok(Some(format!("{field} {ope} {value}")))
    }
}

impl SqlParse for BetweenParse<'_> {
    fn parse(&self) -> Result<Option<String>, CtsError> {
        let data = self.0;
        if data.len() < 4 {
            return Err(FilterError("参数错误，参数长度不够".to_string()));
        }
        // 操作符
        let first = &data[0];
        // 解析操作符
        let ope = handler_name(first)?;
        // 字段
        let second = &data[1];
        let field = handler_name(second)?;
        // 值1
        let third = &data[2];
        let value1 = handler_between_value(third)?;
        // 值2
        let fourth = &data[3];
        let value2 = handler_between_value(fourth)?;

        Ok(Some(format!(" {field} {ope} {value1} and {value2} ")))
    }
}

fn handler_between_value(data: &CtsValue) -> Result<String, CtsError> {
    match data {
        CtsValue::Single(value) => match value {
            Single::Integer(value_int) => Ok(format!("{value_int}")),
            Single::Double(value_double) => Ok(format!("{value_double}")),
            Single::String(value_string) => Ok(format!("'{value_string}'")),
            _ => Err(FilterError("BETWEEN参数错误".to_string())),
        },
        CtsValue::Array(_) => Err(FilterError("BETWEEN参数错误".to_string())),
    }
}

fn handler_in_value(data: &CtsValue) -> Result<String, CtsError> {
    match data {
        CtsValue::Single(value) => match value {
            Single::String(value_str) => match value_str.is_empty() {
                true => Err(FilterError("数据不能为空".to_string())),
                false => Ok(format!("'{value_str}'")),
            },
            Single::Integer(value_int) => Ok(format!("{value_int}")),
            Single::Double(value_double) => Ok(format!("{value_double}")),
            Single::Bool(value_bool) => Ok(format!("{value_bool}")),
        },
        CtsValue::Array(arr) => {
            let mut result = Vec::new();
            for item in arr.iter() {
                let value = handler_in_value(item)?;
                result.push(value);
            }
            Ok(result.join(","))
        }
    }
}

fn handler_like_value(data: &CtsValue) -> Result<String, CtsError> {
    match data {
        CtsValue::Single(value) => match value {
            Single::String(value_str) => match value_str.is_empty() {
                true => Err(FilterError("数据不能为空".to_string())),
                false => Ok(format!("'{value_str}'")),
            },
            _ => Err(FilterError("LIKE参数错误".to_string())),
        },
        CtsValue::Array(_) => Err(FilterError("LIKE参数错误".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::parse::filter::inclusion::{BetweenParse, InParse, LikeParse};
    use crate::expression::{CtsValue, Single, SqlParse};

    #[test]
    fn between() {
        let param = vec![
            CtsValue::Single(Single::String("not between".to_string())),
            CtsValue::Single(Single::Integer(10)),
            CtsValue::Single(Single::Integer(100)),
        ];

        let aa = BetweenParse(&param).parse().unwrap();
        println!("{}", aa.unwrap());
    }

    #[test]
    fn in_aa() {
        let param = vec![
            CtsValue::Single(Single::String("in".to_string())),
            CtsValue::Single(Single::String("field".to_string())),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("in".to_string())),
                CtsValue::Single(Single::String("in".to_string())),
                CtsValue::Single(Single::Integer(123)),
                CtsValue::Single(Single::String("in".to_string())),
            ]),
        ];

        let aa = InParse(&param).parse().unwrap();
        println!("{}", aa.unwrap());
    }

    #[test]
    fn like() {
        let param = vec![
            CtsValue::Single(Single::String("like".to_string())),
            CtsValue::Single(Single::String("name".to_string())),
            CtsValue::Single(Single::String("%asdf".to_string())),
        ];

        let aa = LikeParse(&param).parse().unwrap();
        println!("{}", aa.unwrap());
    }
}
