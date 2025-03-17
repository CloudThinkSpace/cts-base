use crate::error::CtsError;
use crate::expression::SqlParse;

/// 分组参数解析
/// ```sql
/// [field,field2]
/// ```
pub struct GroupByParse<'a>(pub &'a Option<Vec<String>>);

impl SqlParse for GroupByParse<'_> {
    fn parse(&self) -> Result<Option<String>, CtsError> {
        let group_param = self.0;
        match group_param {
            None => Ok(None),
            Some(data) => {
                let group_by = data.join(",");
                Ok(Some(group_by))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_group() {
        let groups = Some(vec!["aaa".to_string(), "bbb".to_string()]);
        let group_parse = GroupByParse(&groups);
        let aa = group_parse.parse().unwrap();
        print!("{}", aa.unwrap());
    }
}
