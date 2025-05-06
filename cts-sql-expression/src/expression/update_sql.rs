use std::collections::HashMap;

use serde_json::Value;
use sqlx::{Pool, Postgres};

/// update sql构造器
/// @param 请求参数
/// @param pool 数据库连接池
/// @table 查询表名
/// @schema 对应数据库的schema
/// @id 数据唯一字段
pub struct UpdateSqlBuilder<'a> {
    data: HashMap<String, Value>,
    pool: &'a Pool<Postgres>,
    table: String,
    schema: String,
    id: String,
}

impl<'a> UpdateSqlBuilder<'a> {
    pub fn new(
        id: String,
        data: HashMap<String, Value>,
        pool: &'a Pool<Postgres>,
        table: String,
        schema: String,
    ) -> Self {
        Self {
            id,
            data,
            pool,
            table,
            schema,
        }
    }
}

impl<'a> UpdateSqlBuilder<'a> {
    pub fn build(&self) -> String {
        let mut sql = format!("UPDATE {}.{} SET ", self.schema, self.table);

        for (key, value) in &self.data {
            sql.push_str(&format!("{} = {}, ", key, handler_value(value)));
        }

        sql.pop();
        sql.pop();
        sql.push_str(&format!(" WHERE id = '{}'", self.id));

        sql
    }

    pub async fn execute(&self) -> Result<u64, sqlx::Error> {
        let sql = self.build();
        let result = sqlx::query(&sql).execute(self.pool).await?;
        Ok(result.rows_affected())
    }
}

/// 处理数据，判断值的类型，返回不同的字符串
pub fn handler_value(data: &Value) -> String {
    match data {
        Value::String(s) => format!("'{}'", s),
        Value::Number(n) => format!("{}", n),
        Value::Bool(b) => format!("{}", b),
        _ => format!("NULL"),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_update_sql_builder() {
        let mut sql = format!("UPDATE {}.{} SET ", "public", "files");
        let data = HashMap::from([
            ("id".to_string(), json!(1)),
            ("name".to_string(), Value::String("John".to_string())),
            ("age".to_string(), json!(30)),
        ]);
        for (key, value) in &data {
            sql.push_str(&format!("{} = {}, ", key, handler_value(value)));
        }

        sql.pop();
        sql.pop();
        sql.push_str(&format!(" WHERE id = '{}'", "123"));

        println!("{}",sql);
    }
}
