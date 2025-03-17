use crate::error::CtsError;
use crate::error::CtsError::FilterError;
use crate::expression::parse::handler_name;
use crate::expression::{CtsValue, Single, SqlParse};

pub struct CompareParse<'a>(pub &'a Vec<CtsValue>);

impl SqlParse for CompareParse<'_> {
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

        let value = handler_value(third)?;

        Ok(Some(format!("{field} {ope} {value}")))
    }
}

fn handler_value(data: &CtsValue) -> Result<String, CtsError> {
    let value = match data {
        CtsValue::Single(value) => match value {
            Single::String(value_str) => {
                format!("'{value_str}'")
            }
            Single::Integer(value_int) => {
                format!("{value_int}")
            }
            Single::Double(value_double) => {
                format!("{value_double}")
            }
            Single::Bool(value_boole) => {
                format!("{value_boole}")
            }
        },
        CtsValue::Array(_) => {
            return Err(FilterError("参数错误".to_string()));
        }
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use crate::expression::parse::filter::compare::CompareParse;
    use crate::expression::{CtsValue, Single, SqlParse};
    #[test]
    fn compare() {
        let param = vec![
            CtsValue::Single(Single::String(">".to_string())),
            CtsValue::Single(Single::String("aaa".to_string())),
            CtsValue::Single(Single::String("bbb".to_string())),
        ];

        let aa = CompareParse(&param).parse().unwrap();
        println!("{}", aa.unwrap())
    }
}
