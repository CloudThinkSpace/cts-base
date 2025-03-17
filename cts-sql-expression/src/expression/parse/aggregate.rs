use crate::error::CtsError;
use crate::error::CtsError::AggregateError;
use crate::expression::{CtsValue, Single, SqlParse};

static OPERATORS: [&str; 5] = ["sum", "count", "max", "min", "avg"];

pub struct AggregateParse<'a>(pub &'a Option<Vec<CtsValue>>);

impl SqlParse for AggregateParse<'_> {
    fn parse(&self) -> Result<Option<String>, CtsError> {
        let aggregates = self.0;

        match aggregates {
            None => Ok(None),
            Some(data) => {
                // 解析统计函数
                let aggregate = handler_parse(data)?;
                Ok(aggregate)
            }
        }
    }
}

fn handler_parse(data: &[CtsValue]) -> Result<Option<String>, CtsError> {
    let mut result = Vec::new();
    // 判断是否为空
    if data.is_empty() {
        return Err(AggregateError("统计参数错误".to_string()));
    }
    // 遍历数据
    for datum in data.iter() {
        match datum {
            CtsValue::Single(ope) => {
                // 判断类型是否为字符串，其他类型抛出错误
                return match ope {
                    Single::String(ope_str) => {
                        // 如果是字符串，该项为操作符，判断操作函数是否支持
                        if !OPERATORS.contains(&ope_str.to_lowercase().as_str()) {
                            return Err(AggregateError("统计操作符不支持".to_string()));
                        }
                        // 判断数组是否等于2
                        match data.len() {
                            2 => {
                                let second = &data[1];
                                match second {
                                    CtsValue::Single(field) => match field {
                                        Single::String(field_str) => Ok(Some(format!(
                                            "{ope_str}({field_str}) as {field_str}"
                                        ))),
                                        _ => Err(AggregateError("统计【字段】必须为字符串".to_string())),
                                    },
                                    CtsValue::Array(_) => Err(AggregateError("统计参数错误".to_string())),
                                }
                            }
                            3 => {
                                let second = &data[1];
                                match second {
                                    CtsValue::Single(field) => match field {
                                        Single::String(field_str) => {
                                            let third = &data[2];
                                            match third {
                                                CtsValue::Single(value) => match value {
                                                    Single::String(value_str) => Ok(Some(format!(
                                                        "{ope_str}({field_str}) as {value_str}"
                                                    ))),
                                                    _ => {
                                                        Err(AggregateError("统计【别名】必须为字符串".to_string()))
                                                    }
                                                },
                                                CtsValue::Array(_) => {
                                                    Err(AggregateError("统计参数错误".to_string()))
                                                }
                                            }
                                        }
                                        _ => Err(AggregateError("统计【字段】必须为字符串".to_string())),
                                    },
                                    CtsValue::Array(_) => Err(AggregateError("统计参数错误".to_string())),
                                }
                            }
                            _ => Err(AggregateError("统计参数错误".to_string())),
                        }
                    }
                    _ => Err(AggregateError("统计操作符不支持".to_string())),
                };
            }
            CtsValue::Array(sub_data) => {
                let expression = handler_parse(sub_data)?;
                if let Some(expr) = expression {
                    result.push(expr);
                }
            }
        }
    }

    Ok(Some(result.join(",")))
}

#[cfg(test)]
mod tests {
    use crate::expression::parse::aggregate::AggregateParse;
    use crate::expression::{CtsValue, Single, SqlParse};

    #[test]
    fn parse_aa() {
        let aaa = Some(vec![
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("sum".to_string())),
                CtsValue::Single(Single::String("field".to_string())),
                CtsValue::Single(Single::String("aaaa".to_string())),
            ]),
            CtsValue::Array(vec![
                CtsValue::Single(Single::String("sum".to_string())),
                CtsValue::Single(Single::String("field".to_string())),
            ]),
        ]);
        let bbb = AggregateParse(&aaa);
        let cc = bbb.parse().unwrap();
        println!("{}", cc.unwrap());
    }
}
