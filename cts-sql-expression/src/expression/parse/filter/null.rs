use crate::error::CtsError;
use crate::error::CtsError::FilterError;
use crate::expression::parse::handler_name;
use crate::expression::{CtsValue, SqlParse};

pub struct NullParse<'a>(pub &'a Vec<CtsValue>);

impl SqlParse for NullParse<'_> {
    fn parse(&self) -> Result<Option<String>, CtsError> {
        let data = self.0;
        if data.len() < 2 {
            return Err(FilterError("参数错误，参数长度不够".to_string()));
        }
        // 操作符
        let first = &data[0];
        // 解析操作符
        let ope = handler_name(first)?;
        // 值
        let second = &data[1];
        let field = handler_name(second)?;

        Ok(Some(format!("{field} {ope} ")))
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::parse::filter::null::NullParse;
    use crate::expression::{CtsValue, Single, SqlParse};

    #[test]
    fn null_parse() {
        let aa = vec![
            CtsValue::Single(Single::String("is null".to_string())),
            CtsValue::Single(Single::String("aa".to_string())),
        ];
        let bb = NullParse(&aa).parse().unwrap();
        println!("{}", bb.unwrap())
    }
}
