use crate::convert::PgRowConvert;
use crate::expression::GEOMETRY;
use crate::response::CtsResult;
use cts_pgrow::SerMapPgRow;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};


/// geojson转换工具
/// > 将CstResult 转换成FeatureCollection
/// - json数组
/// ```json
///     {
///         "type": "FeatureCollection",
///         "features":[
///             {
///                 "type": "Point",
///                 "geometry": Point,
///                 "properties":{
///                     "name":"aaa",
///                     "age":1
///                 },
///                 "id": 1,
///             }
///         ]
///      }
/// ```
pub struct GeoJsonConvert;

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
    fn convert(&self, data: CtsResult) -> Value {
        handler_result(data)
    }
}

fn handler_result(data: CtsResult) -> Value {
    match data {
        CtsResult::Single(single) => {
            let mut result = Vec::new();
            result.push(single);
            handler_result(CtsResult::List(result))
        }
        CtsResult::List(list) => {
            let mut result = Vec::new();
            for (index, row) in list.into_iter().enumerate() {
                let row_map = SerMapPgRow::from(row);
                let value = row_map.into();
                let mut geometry = Value::String("".to_string());
                let mut properties = Map::new();
                // 判断是否为对象
                if let Value::Object(map) = value {
                    // 遍历map对象
                    for (key, value) in map.into_iter() {
                        // 判断是否为空间字段
                        if key == GEOMETRY {
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
        CtsResult::Page(page) => {
            let list = page.list;
            handler_result(CtsResult::List(list))
        }
    }
}
