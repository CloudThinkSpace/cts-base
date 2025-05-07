use std::collections::HashMap;

use chrono::{naive, Local};
use serde_json::Value;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::{CREATED_AT, UPDATED_AT};

/// save sql构造器
/// @param 请求参数
/// @param pool 数据库连接池
/// @table 查询表名
/// @schema 对应数据库的schema
pub struct SaveSqlBuilder<'a> {
    data: HashMap<String, Value>,
    pool: &'a Pool<Postgres>,
    table: String,
    schema: String,
}

impl<'a> SaveSqlBuilder<'a> {
    pub fn new(
        mut data: HashMap<String, Value>,
        pool: &'a Pool<Postgres>,
        table: String,
        schema: String,
    ) -> Self {
        // 创建
        let uuid_str = Uuid::new_v4().to_string();
        // 插入id字段，如果存在，替换成uuid字符串
        data.insert("id".to_string(), Value::String(uuid_str));
        // 插入日期字段
        let date = Local::now().to_string();
        data.insert(CREATED_AT.to_string(), Value::String(date.to_string()));
        data.insert(UPDATED_AT.to_string(), Value::String(date));
        Self {
            data,
            pool,
            table,
            schema,
        }
    }

    pub fn build(&self) -> String {
        // 插入sql字符串
        let mut sql = format!("INSERT INTO {}.{} (", self.schema, self.table);
        // 结果字符串
        let mut values = String::new();
        // 遍历字段
        for (key, value) in &self.data {
            // 收集sql字段
            sql.push_str(&format!("\"{}\", ", key));
            // 收集sql值
            values.push_str(&format!("{}, ", handler_value(value)));
        }
        // 弹出逗号和空格
        values.pop();
        values.pop();
        // 弹出逗号和空格
        sql.pop();
        sql.pop();
        sql.push_str(") VALUES (");
        sql.push_str(&values);
        sql.push_str(")");
        sql
    }

    pub async fn execute(&self) -> Result<u64, sqlx::Error> {
        let sql = self.build();
        let result = sqlx::query(&sql).execute(self.pool).await?;
        Ok(result.rows_affected())
    }
}

/// 处理数据，判断值的类型，返回不同的字符串
fn handler_value(data: &Value) -> String {
    match data {
        Value::String(s) => format!("'{}'", s),
        Value::Number(n) => format!("{}", n),
        Value::Bool(b) => format!("{}", b),
        _ => format!("NULL"),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::{json, Value};

    use super::handler_value;

    #[test]
    fn test_build() {
        let data = HashMap::from([
            ("id".to_string(), json!(1)),
            ("name".to_string(), Value::String("John".to_string())),
            ("age".to_string(), json!(30)),
        ]);
        // 插入sql字符串
        let mut sql = format!("INSERT INTO {}.{} (", "public", "files");
        // 结果字符串
        let mut values = String::new();
        for (key, value) in &data {
            sql.push_str(&format!("\"{}\", ", key));
            values.push_str(&format!("{}, ", handler_value(value)));
        }
        values.pop();
        values.pop();
        sql.pop();
        sql.pop();
        sql.push_str(") VALUES (");
        sql.push_str(&values);
        sql.push_str(")");
        println!("{}", sql);
    }
}
