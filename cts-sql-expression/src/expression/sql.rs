use crate::config::ExpressionConfig;
use crate::error::CtsError;
use crate::error::CtsError::ParamError;
use crate::expression::parse::aggregate::AggregateParse;
use crate::expression::parse::field::FieldParse;
use crate::expression::parse::filter::FilterParse;
use crate::expression::parse::group::GroupByParse;
use crate::expression::parse::order::OrderByParse;
use crate::expression::parse::page::PageParse;
use crate::expression::query_builder::QueryBuilder;
use crate::expression::{Course, SqlParse};
use crate::request::{CtsParam, GeometryFormat};
use crate::response::{CtsResult, PageValue};
use cts_pgrow::SerMapPgRow;
use serde_json::Value;
use sqlx::{Pool, Postgres, Row};

/// sql构造器
/// @param 请求参数
/// @param pool 数据库连接池
/// @table 查询表名
/// @schema 对应数据库的schema
/// @geometry 空间字段名称
pub struct SqlBuilder<'a> {
    param: CtsParam,
    pool: &'a Pool<Postgres>,
    table: String,
    schema: String,
    geometry: Option<String>,
}

impl<'a> SqlBuilder<'a> {
    pub fn new(
        pool: &'a Pool<Postgres>,
        table: String,
        config: ExpressionConfig,
        param: CtsParam,
    ) -> Self {
        let new_schema = config.schema();
        let param = match config.search {
            true => {
                param.search_param()
            }
            false => {
                param
            }
        };
        Self {
            param,
            table,
            pool,
            geometry: config.geometry,
            schema: new_schema,
        }
    }

    pub fn new_simplify(
        pool: &'a Pool<Postgres>,
        table: String,
        config: ExpressionConfig,
        param: CtsParam,
        id: String,
    ) -> Self {
        // 简化参数
        let mut param = param.query_param(id);
        if config.search {
            param.geo_format = None;
            param.return_geometry = None;
        }
        let new_schema = config.schema();
        Self {
            param,
            table,
            pool,
            geometry: None,
            schema: new_schema,
        }
    }

    // 解析查询sql函数
    async fn parse(&self) -> Result<String, CtsError> {
        let param = &self.param;
        // filter 解析
        let filter = FilterParse(&param.filter).parse()?;
        // group 解析
        let group = GroupByParse(&param.group_by).parse()?;
        // field 解析
        let field = FieldParse(&param.out_fields).parse()?;
        // aggregate 解析
        let aggregate = AggregateParse(&param.aggregate).parse()?;
        // order by 解析
        let order = OrderByParse(&param.order).parse()?;
        // page 分页解析
        let page = PageParse(&param.page).parse()?;
        // sql构造对象
        let mut builder = QueryBuilder::new_select();
        // table表名
        let table = &self.table;
        // schema 模式
        let schema = &self.schema;
        // 判断是否有分组统计
        let fields = match &group {
            None => {
                // 匹配是否有字段
                match &field {
                    // 没有字段时需要查询表字段，并且过滤掉空间字段
                    None => self.get_table_columns().await?,
                    Some(fields) => {
                        // 匹配是否返回空间字段，true单独处理空间字段
                        match param.return_geometry {
                            Some(data) if data => {
                                // 判断是否有空间字段
                                match &self.geometry {
                                    None => fields.to_string(),
                                    Some(geometry_field) => {
                                        let geometry_field =
                                            self.handler_geometry_format(geometry_field);
                                        format!("{fields},{geometry_field}")
                                    }
                                }
                            }
                            _ => fields.to_string(),
                        }
                    }
                }
            }
            Some(groups) => {
                // 判断字段是否有数据
                match field {
                    None => {
                        // 处理统计函数Ï
                        match &aggregate {
                            None => {
                                // 解析
                                groups.to_string()
                            }
                            Some(agg) => agg.to_string(),
                        }
                    }
                    Some(field_str) => match aggregate {
                        None => field_str,
                        Some(agg) => {
                            format!("{field_str}, {agg}")
                        }
                    },
                }
            }
        };
        builder.push(fields);
        builder.push(" from ");
        builder.push(schema);
        builder.push(".");
        builder.push(table);

        // 判断是否有过滤条件
        if let Some(data) = filter {
            builder.push(" where ");
            builder.push(data);
        }
        // 处理group
        if let Some(data) = group {
            builder.push(" group by ");
            builder.push(data);
        }

        // 排序字段
        if let Some(data) = order {
            builder.push(" order by ");
            builder.push(data);
        }

        // 分页
        if let Some(data) = page {
            builder.push(data);
        }

        Ok(builder.build())
    }

    // 查询表字段方法
    async fn get_table_columns(&self) -> Result<String, CtsError> {
        let param = &self.param;
        let table = &self.table;
        let schema = &self.schema;
        match &self.geometry {
            None => Ok("*".to_string()),
            Some(geometry_field) => {
                // 查询表字段
                let pool = self.pool;
                let query_columns = format!("SELECT column_name,data_type,is_nullable,column_default FROM information_schema.columns WHERE table_schema = '{}' AND table_name   = '{}'", schema, table);
                let result = sqlx::query_as::<_, Course>(&query_columns)
                    .fetch_all(pool)
                    .await;
                let result = result.map_err(|err| ParamError(format!("{err}")))?;
                let mut result_vec = Vec::new();
                // 收集表名称
                for row in result {
                    result_vec.push(row.column_name);
                }
                // 排除空间字段
                let mut fields: Vec<String> = result_vec
                    .into_iter()
                    .filter(|item| item != geometry_field)
                    .collect();
                // 判断是返回空间字段
                if let Some(data) = param.return_geometry {
                    if data {
                        // 处理空间字段
                        let geometry_field = self.handler_geometry_format(geometry_field);
                        fields.push(geometry_field);
                    }
                }
                Ok(fields.join(","))
            }
        }
    }

    // 解析分页查询sql函数
    async fn parse_page_count(&self) -> Result<String, CtsError> {
        let param = &self.param;
        // filter 解析
        let filter = FilterParse(&param.filter).parse()?;
        let mut builder = QueryBuilder::new("select count(*) as count");
        let table = &self.table;
        let schema = &self.schema;
        builder.push(" from ");
        builder.push(schema);
        builder.push(".");
        builder.push(table);
        // 处理过滤
        if let Some(data) = filter {
            builder.push(" where ");
            builder.push(data);
        }
        Ok(builder.build())
    }

    /// 处理geometry format 格式参数，根据不同的格式参数，返回不同的空间字段
    fn handler_geometry_format(&self, geometry_field: &str) -> String {
        let param = &self.param;
        let geo_format = &param.geo_format;
        // 添加空间查询字段
        match geo_format {
            None => {
                // 将空间字段转换成字符串wkt格式字符串
                format!(" st_asewkt({geometry_field}) as {geometry_field} ")
            }
            Some(format) => match format {
                GeometryFormat::GeoJson => {
                    format!("st_asgeojson({geometry_field}) as {geometry_field} ")
                }
                GeometryFormat::WKT => {
                    format!("st_asewkt({geometry_field}) as {geometry_field} ")
                }
                GeometryFormat::Byte => {
                    format!("st_asbinary({geometry_field}) as {geometry_field} ")
                }
                GeometryFormat::Text => {
                    format!("st_astext({geometry_field}) as {geometry_field} ")
                }
                GeometryFormat::WKB => {
                    format!("st_asewkb({geometry_field}) as {geometry_field} ")
                }
            },
        }
    }

    pub async fn query(&self) -> Result<CtsResult, CtsError> {
        // 解析查询语句
        let query = self.parse().await?;
        // 查询数据
        let list = sqlx::query(&query)
            .fetch_all(self.pool)
            .await
            .map_err(|err| ParamError(err.to_string()))?;
        // 判断是否有分组条件，有分组条件不能进行分页
        if self.param.group_by.is_none() {
            // 分页查询
            if let Some(page_param) = &self.param.page {
                // 解析分页查询语句
                let query = self.parse_page_count().await?;
                // 查询分页结果
                let result = sqlx::query(&query)
                    .fetch_one(self.pool)
                    .await
                    .map_err(|err| ParamError(err.to_string()))?;
                let total = result.get::<i64, _>(0);
                // 计算页数
                let pages = (total as f64 * 1.0 / page_param.page_size as f64).ceil() as i64;
                let page_value = PageValue {
                    pages,
                    current_page: page_param.page,
                    page_size: page_param.page_size,
                    total,
                    list,
                };
                Ok(CtsResult::Page(page_value))
            } else {
                // 返回成功数据列表
                Ok(CtsResult::List(list))
            }
        } else {
            // 返回成功数据列表
            Ok(CtsResult::List(list))
        }
    }

    pub async fn query_one(&self) -> Result<Value, CtsError> {
        // 解析查询语句
        let query = self.parse().await?;
        // 查询数据
        let row = sqlx::query(&query)
            .fetch_one(self.pool)
            .await
            .map_err(|err| ParamError(err.to_string()))?;
        let row_map = SerMapPgRow::from(row);
        let value = row_map.into();
        Ok(value)
    }
}
