use serde::{Deserialize, Serialize};
use crate::expression::{CtsValue, Single};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CtsParam {
    pub filter: Option<Vec<CtsValue>>,
    pub group_by: Option<Vec<String>>,
    pub out_fields: Option<Vec<CtsValue>>,
    pub aggregate: Option<Vec<CtsValue>>,
    pub return_geometry: Option<bool>,
    pub order: Option<Vec<CtsValue>>,
    pub page: Option<PageParam>,
    pub geo_format: Option<GeometryFormat>,
    pub format: Option<CtsFormat>,
}


impl CtsParam {
    pub fn search_param(mut self) -> Self {
        self.group_by = None;
        self.aggregate = None;
        self.return_geometry = None;
        self.geo_format = None;
        self
    }

    pub fn query_param(mut self, id:String) -> Self {
        // 清理过滤参数
        self.filter = None;
        self.group_by = None;
        self.aggregate = None;
        self.page = None;
        //重新设置条件
        self.filter = Some(vec![
            CtsValue::Single(Single::String("=".to_string())),
            CtsValue::Single(Single::String("id".to_string())),
            CtsValue::Single(Single::String(id)),
        ]);
        self
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageParam {
    pub page: i32,
    pub page_size: i32,
}



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CtsFormat {
    Json,
    GeoJson,
    CSV,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GeometryFormat {
    GeoJson,
    WKT,
    Byte,
    Text,
    WKB,
}