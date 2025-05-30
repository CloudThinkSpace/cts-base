use crate::config::{ExpressionConfig, QueryMode};
use crate::error::CtsError;
use crate::error::CtsError::ParamError;
use crate::expression::parse::aggregate::AggregateParse;
use crate::expression::parse::field::FieldParse;
use crate::expression::parse::filter::FilterParse;
use crate::expression::parse::group::GroupByParse;
use crate::expression::parse::order::OrderByParse;
use crate::expression::parse::page::PageParse;
use crate::expression::query_builder::QueryBuilder;
use crate::expression::{Course, SqlParse, GEOMETRY};
use crate::request::{CtsFormat, CtsParam, GeometryFormat};
use crate::response::{CtsResult, PageValue};
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
    query_mode: QueryMode,
}

impl<'a> SqlBuilder<'a> {
    pub fn new(
        pool: &'a Pool<Postgres>,
        table: String,
        config: ExpressionConfig,
        param: CtsParam,
    ) -> Self {
        let new_schema = config.schema();
        let param = match config.query_mode {
            QueryMode::Normal => {
                // 设置查询参数，去掉空间查询相关参数
                param.search_param()
            }
            QueryMode::Spatial => param,
        };
        Self {
            param,
            table,
            pool,
            query_mode: config.query_mode,
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
        // 如果是普通查询，取消空间格式参数
        if let QueryMode::Normal = config.query_mode {
            param.geo_format = None;
            param.return_geometry = None;
        }
        // 获取数据库设计模式，默认public
        let new_schema = config.schema();
        // 创建builder对象
        Self {
            param,
            table,
            pool,
            query_mode: config.query_mode,
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
        let order = OrderByParse(&param.order_by).parse()?;
        // page 分页解析
        let page = PageParse(&param.page).parse()?;
        // sql构造对象
        let mut builder = QueryBuilder::new_select();
        // table表名
        let table = &self.table;
        // schema 模式
        let schema = &self.schema;
        // 判断是否有统计参数
        let fields = match &aggregate {
            None => {
                // 匹配是否有字段
                match &field {
                    // 没有字段时需要查询表字段
                    None => self.get_table_columns().await?,
                    Some(fields) => {
                        // 判断模式
                        match self.query_mode {
                            QueryMode::Normal => fields.to_string(),
                            QueryMode::Spatial => {
                                // 匹配是否返回空间字段，true单独处理空间字段
                                match param.return_geometry {
                                    Some(data) if data => {
                                        // 获取空间字段
                                        let geometry = self.get_table_geometry().await;
                                        // 判断是否有空间字段
                                        let geometry_field = &geometry.ok_or(ParamError(
                                            "参数错误，该数据不包含空间字段".to_string(),
                                        ))?;
                                        let geometry_field =
                                            self.handler_geometry_format(geometry_field);
                                        format!("{fields},{geometry_field}")
                                    }
                                    _ => fields.to_string(),
                                }
                            }
                        }
                    }
                }
            }
            Some(agg) => {
                // 判断字段是否有数据
                match field {
                    None => agg.to_string(),
                    Some(field_str) => {
                        format!("{field_str}, {agg}")
                    }
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
        // 判断查询方式是那种
        match &self.query_mode {
            QueryMode::Normal => Ok("*".to_string()),
            QueryMode::Spatial => {
                // 查询表字段
                let pool = self.pool;
                let query_columns = format!("SELECT column_name,udt_name FROM information_schema.columns WHERE table_schema = '{}' AND table_name   = '{}'", schema, table);
                // 查询表字段
                let result = sqlx::query_as::<_, Course>(&query_columns)
                    .fetch_all(pool)
                    .await;
                // 获取表字段列表
                let result = result.map_err(|err| ParamError(format!("{err}")))?;
                // 空间字段
                let mut geometry_field = None;
                // 字段列表
                let mut fields = Vec::new();
                // 遍历字段并收集字段名称
                for item in result.into_iter() {
                    // 判断是否有空间字段
                    if item.udt_name == "geometry" {
                        geometry_field = Some(item.column_name);
                    } else {
                        fields.push(item.column_name);
                    }
                }
                // 判断是返回空间字段
                if matches!(param.return_geometry, Some(true)) {
                    // 判断是否返回空间字段
                    let geom = geometry_field
                        .ok_or(ParamError("参数错误，该数据不包含空间字段".to_string()))?;
                    // 处理空间字段
                    let geometry_field = self.handler_geometry_format(geom.as_str());
                    // 添加空间字段
                    fields.push(geometry_field);
                }
                Ok(fields.join(","))
            }
        }
    }

    // 查询表空间字段，可能有或者没有空间字段，返回option类型
    async fn get_table_geometry(&self) -> Option<String> {
        let table = &self.table;
        let schema = &self.schema;
        // 查询表字段
        let pool = self.pool;
        let query_columns = format!("SELECT column_name,udt_name FROM information_schema.columns WHERE table_schema = '{}' AND table_name = '{}' AND udt_name='geometry'", schema, table);
        // 查询表字段
        let result = sqlx::query_as::<_, Course>(&query_columns)
            .fetch_one(pool)
            .await;

        match result {
            Ok(data) => Some(data.column_name),
            Err(_) => None,
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
                format!("st_asgeojson({geometry_field}) as {GEOMETRY} ")
            }
            Some(format) => match format {
                GeometryFormat::GeoJson => {
                    format!("st_asgeojson({geometry_field}) as {GEOMETRY} ")
                }
                GeometryFormat::WKT => {
                    format!("st_asewkt({geometry_field}) as {GEOMETRY} ")
                }
                GeometryFormat::Byte => {
                    format!("st_asbinary({geometry_field}) as {GEOMETRY} ")
                }
                GeometryFormat::Text => {
                    format!("st_astext({geometry_field}) as {GEOMETRY} ")
                }
                GeometryFormat::WKB => {
                    format!("st_asewkb({geometry_field}) as {GEOMETRY} ")
                }
            },
        }
    }

    pub async fn query(&mut self) -> Result<Value, CtsError> {
        // 格式处理
        let format = self.format();
        // 解析查询语句
        let query = self.parse().await?;
        // 查询数据
        let list = sqlx::query(&query)
            .fetch_all(self.pool)
            .await
            .map_err(|err| ParamError(err.to_string()))?;
        // 判断是否有统计条件，有统计条件不能进行分页
        if self.param.aggregate.is_none() {
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
                Ok(CtsResult::Page(page_value).to_value(format))
            } else {
                // 返回成功数据列表
                Ok(CtsResult::List(list).to_value(format))
            }
        } else {
            // 返回成功数据列表
            Ok(CtsResult::List(list).to_value(format))
        }
    }

    pub async fn query_one(&mut self) -> Result<Value, CtsError> {
        // 格式处理
        let format = self.format();
        // 解析查询语句
        let query = self.parse().await?;
        // 查询数据
        let row = sqlx::query(&query)
            .fetch_one(self.pool)
            .await
            .map_err(|err| ParamError(err.to_string()))?;

        Ok(CtsResult::Single(row).to_value(format))
    }

    fn format(&mut self) -> CtsFormat {
        // 判断格式
        match &self.param.format {
            None => CtsFormat::Json,
            Some(data) => {
                match data {
                    CtsFormat::Json => CtsFormat::Json,
                    CtsFormat::GeoJson => {
                        // 设置geojson格式返回
                        self.param.geo_format = Some(GeometryFormat::GeoJson);
                        // 设置返回空间字段
                        self.param.return_geometry = Some(true);
                        CtsFormat::GeoJson
                    }
                    CtsFormat::CSV => CtsFormat::CSV,
                }
            }
        }
    }
}
