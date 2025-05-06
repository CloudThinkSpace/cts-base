use sqlx::{Pool, Postgres};


/// delete sql构造器
/// @param 请求参数
/// @param pool 数据库连接池
/// @table 查询表名
/// @schema 对应数据库的schema
/// @id 数据唯一字段
pub struct DeleteSqlBuilder<'a> {
    pool: &'a Pool<Postgres>,
    table: String,
    schema: String,
    id: String,
}

impl<'a> DeleteSqlBuilder<'a> {
    pub fn new(pool: &'a Pool<Postgres>, table: String, schema: String, id: String) -> Self {
        Self {
            pool,
            table,
            schema,
            id,
        }
    }

    pub fn build(&self) -> String {
        let sql = format!(
            "DELETE FROM {}.{} WHERE id = $1",
            self.schema, self.table
        );
        sql
    }

    pub async fn execute(&self) -> Result<(), sqlx::Error> {
        let sql = self.build();
        sqlx::query(&sql)
            .bind(&self.id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
