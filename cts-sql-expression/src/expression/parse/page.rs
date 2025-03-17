use crate::error::CtsError;
use crate::expression::{PageParam, SqlParse};

/// 解析分页参数
/// @param page 默认 1
/// @param page_size 默认 10
pub struct PageParse<'a>(pub &'a Option<PageParam>);

impl SqlParse for PageParse<'_> {
    fn parse(&self) -> Result<Option<String>, CtsError> {
        let page_param = self.0;
        match page_param {
            None => Ok(None),
            Some(data) => {
                let page = &data.page;
                let page_size = &data.page_size;
                let offset = (page - 1) * page_size;
                Ok(Some(format!("LIMIT {page_size} OFFSET {offset}")))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_group() {
        let groups = Some(PageParam {
            page: 1,
            page_size: 10,
        });
        let group_parse = PageParse(&groups);
        let aa = group_parse.parse().unwrap();
        print!("{}", aa.unwrap());
    }
}
