pub mod compare;
pub mod inclusion;
pub mod logic;
pub mod null;

use crate::error::CtsError;
use crate::error::CtsError::FilterError;
use crate::expression::parse::filter::compare::CompareParse;
use crate::expression::parse::filter::inclusion::{BetweenParse, InParse, LikeParse};
use crate::expression::parse::filter::logic::{NotParse, OrAndParse};
use crate::expression::parse::filter::null::NullParse;
use crate::expression::parse::handler_name;
use crate::expression::{CtsValue, SqlParse};
/// 过滤条件解析
/// ```sql
/// [express,field,value]
/// ["or",["=",field,value],["=",field2,value]]
/// ["like",field,"%aaaa"]
/// ["in",field,[value1,value2]]
/// ["between",field,value1,valu2]
/// ```
pub struct FilterParse<'a>(pub &'a Option<Vec<CtsValue>>);

impl SqlParse for FilterParse<'_> {
    fn parse(&self) -> Result<Option<String>, CtsError> {
        let filters = self.0;
        match filters {
            None => Ok(None),
            Some(data) => {
                // 解析过滤参数
                let filter = filter_parse(data)?;
                Ok(filter)
            }
        }
    }
}

pub fn filter_parse(data: &Vec<CtsValue>) -> Result<Option<String>, CtsError> {
    // 判断数组长度是否小于3
    if data.len() < 2 {
        return Err(FilterError("过滤参数长度不够，至少2位。".to_string()));
    }
    let first = &data[0];
    let ope = handler_name(first)?;
    //
    let expression = match ope.to_lowercase().as_str() {
        ">" | "<" | ">=" | "<=" | "=" | "!=" => CompareParse(data).parse()?,
        "or" | "and" | "" => OrAndParse(data).parse()?,
        "not" => NotParse(data).parse()?,
        "in" | "not in" => InParse(data).parse()?,
        "between" | "not between" => BetweenParse(data).parse()?,
        "like" => LikeParse(data).parse()?,
        "is null" | "is not null" => NullParse(data).parse()?,
        _ => return Err(FilterError("过滤参数错误".to_string())),
    };
    Ok(expression)
}

#[cfg(test)]
mod tests {
    use crate::expression::{CtsValue, Single};
    use crate::expression::parse::filter::filter_parse;

    #[test]
    fn sql_parse_test() {
        let data = vec![
            CtsValue::Single(Single::String("or".to_string())),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("=".to_string())),
                CtsValue::Single(Single::String("field1".to_string())),
                CtsValue::Single(Single::String("aa".to_string())),
            ]),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("like".to_string())),
                CtsValue::Single(Single::String("field2".to_string())),
                CtsValue::Single(Single::Integer(123)),
            ]),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String(">".to_string())),
                CtsValue::Single(Single::String("field2".to_string())),
                CtsValue::Single(Single::String("bb".to_string())),
            ]),
        ];
        let aa = filter_parse(&data).unwrap();
        println!("{}", aa.unwrap())
    }
}
