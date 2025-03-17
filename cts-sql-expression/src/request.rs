use serde::{Deserialize, Serialize};
use crate::expression::CtsValue;

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