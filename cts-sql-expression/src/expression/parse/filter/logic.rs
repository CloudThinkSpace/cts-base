use crate::error::CtsError;
use crate::error::CtsError::FilterError;
use crate::expression::parse::filter::filter_parse;
use crate::expression::parse::handler_name;
use crate::expression::{CtsValue, SqlParse};

pub struct OrAndParse<'a>(pub &'a Vec<CtsValue>);
pub struct NotParse<'a>(pub &'a Vec<CtsValue>);

impl SqlParse for OrAndParse<'_> {
    fn parse(&self) -> Result<Option<String>, CtsError> {
        let data = self.0;
        if data.len() < 2 {
            return Err(FilterError("参数错误，参数长度不够".to_string()));
        }
        // 操作符
        let first = &data[0];
        // 解析操作符
        let ope = handler_name(first)?;
        let mut result = Vec::new();
        // 遍历
        for datum in data.iter().skip(1) {
            let expression = handler_value(datum)?;
            result.push(format!("({expression})"));
        }
        Ok(Some(result.join(format!(" {ope} ").as_str())))
    }
}

impl SqlParse for NotParse<'_> {
    fn parse(&self) -> Result<Option<String>, CtsError> {
        let data = self.0;
        if data.len() < 2 {
            return Err(FilterError("参数错误，参数长度不够".to_string()));
        }
        // 操作符
        let first = &data[0];
        // 解析操作符
        let ope = handler_name(first)?;
        let mut result = Vec::new();
        // 遍历
        for datum in data.iter().skip(1) {
            let expression = handler_value(datum)?;
            result.push(expression);
        }
        Ok(Some(format!("{ope} ({})", result.join(" and "))))
    }
}

pub fn handler_value(data: &CtsValue) -> Result<String, CtsError> {
    match data {
        CtsValue::Single(_) => Err(FilterError("参数错误".to_string())),
        CtsValue::Array(arr) => {
            // 解析表达式
            let expression = filter_parse(arr)?;
            match expression {
                None => Err(FilterError("参数错误".to_string())),
                Some(data) => Ok(data),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::parse::filter::logic::{NotParse, OrAndParse};
    use crate::expression::{CtsValue, Single, SqlParse};

    #[test]
    fn or_parse() {
        let data = vec![
            CtsValue::Single(Single::String("or".to_string())),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("=".to_string())),
                CtsValue::Single(Single::String("field1".to_string())),
                CtsValue::Single(Single::String("aa".to_string())),
            ]),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("=".to_string())),
                CtsValue::Single(Single::String("field2".to_string())),
                CtsValue::Single(Single::String("bb".to_string())),
            ]),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String(">".to_string())),
                CtsValue::Single(Single::String("field2".to_string())),
                CtsValue::Single(Single::String("bb".to_string())),
            ]),
        ];

        let aa = OrAndParse(&data).parse().unwrap();
        println!("{}", aa.unwrap());
    }
    #[test]
    fn and_parse() {
        let data = vec![
            CtsValue::Single(Single::String("and".to_string())),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("=".to_string())),
                CtsValue::Single(Single::String("field1".to_string())),
                CtsValue::Single(Single::String("aa".to_string())),
            ]),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("=".to_string())),
                CtsValue::Single(Single::String("field2".to_string())),
                CtsValue::Single(Single::String("bb".to_string())),
            ]),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String(">".to_string())),
                CtsValue::Single(Single::String("field2".to_string())),
                CtsValue::Single(Single::String("bb".to_string())),
            ]),
        ];

        let aa = OrAndParse(&data).parse().unwrap();
        println!("{}", aa.unwrap());
    }

    #[test]
    fn and_parse1() {
        let data = vec!["12","23"];

        let aa = data.join("and");
        println!("{}", aa);
    }

    #[test]
    fn not_parse() {
        let data = vec![
            CtsValue::Single(Single::String("not".to_string())),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("=".to_string())),
                CtsValue::Single(Single::String("field1".to_string())),
                CtsValue::Single(Single::String("aa".to_string())),
            ]),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("=".to_string())),
                CtsValue::Single(Single::String("field2".to_string())),
                CtsValue::Single(Single::String("bb".to_string())),
            ]),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String(">".to_string())),
                CtsValue::Single(Single::String("field2".to_string())),
                CtsValue::Single(Single::String("bb".to_string())),
            ]),
        ];

        let aa = NotParse(&data).parse().unwrap();
        println!("{}", aa.unwrap());
    }
}
