use crate::convert::PgRowConvert;
use cts_pgrow::SerMapPgRow;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::postgres::PgRow;

pub struct GeoJsonConvert {
    geom: String,
}
impl GeoJsonConvert {
    pub fn new_geom(geom: String) -> Self {
        Self { geom }
    }

    pub fn new() -> Self {
        Self {
            geom: "geom".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Feature {
    r#type: String,
    geometry: Value,
    properties: Map<String, Value>,
    id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct FeatureCollection {
    r#type: String,
    features: Vec<Feature>,
}

impl FeatureCollection {
    fn new(features: Vec<Feature>) -> Self {
        Self {
            r#type: String::from("FeatureCollection"),
            features,
        }
    }
}

impl PgRowConvert for GeoJsonConvert {
    fn convert(&self, data: Vec<PgRow>) -> Value {
        let mut result = Vec::new();

        for (index, row) in data.into_iter().enumerate() {
            let row_map = SerMapPgRow::from(row);
            let value = row_map.into();
            let mut geometry = Value::String("".to_string());
            let mut properties = Map::new();
            // 判断是否为对象
            if let Value::Object(map) = value {
                // 遍历map对象
                for (key, value) in map.into_iter() {
                    // 判断是否为空间字段
                    if key == self.geom {
                        if let Value::String(data) = value {
                            geometry = serde_json::from_str(&data).unwrap()
                        }
                    } else {
                        // 插入map对象
                        properties.insert(key, value);
                    }
                }
                // 收集feature对象
                result.push(Feature {
                    r#type: "Feature".to_string(),
                    geometry,
                    properties,
                    id: index,
                })
            }
        }
        // 创建对象
        let feature_collection = FeatureCollection::new(result);
        serde_json::to_value(feature_collection).unwrap()
    }
}
