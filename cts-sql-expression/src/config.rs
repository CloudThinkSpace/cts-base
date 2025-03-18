use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct ExpressionConfig {
    pub schema: Option<String>,
    pub geometry: Option<String>,
    pub query_mode: QueryMode,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum QueryMode {
    Normal,
    Spatial,
}

impl ExpressionConfig {
    pub fn new_normal(schema: Option<String>, geometry: Option<String>) -> Self {
        Self { schema, geometry, query_mode: QueryMode::Normal }
    }

    pub fn new(schema: Option<String>, geometry: Option<String>) -> Self {
        Self { schema, geometry, query_mode: QueryMode::Spatial }
    }

    pub fn schema(&self) -> String {
        match &self.schema {
            None => {
                "public".to_string()
            }
            Some(data) => {
                data.to_string()
            }
        }
    }
}