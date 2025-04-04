use serde_json::Value;
use sqlx::postgres::{PgRow, PgValueRef, Postgres};
use sqlx::{Column, Decode, Row, TypeInfo, ValueRef};

use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};

pub fn read_header(row: &PgRow) -> Vec<String> {
    let columns = row.columns();
    let mut headers = vec![];
    for c in columns {
        headers.push(c.name().to_string());
    }
    headers
}

pub fn read_row(row: &PgRow) -> Vec<Value> {
    let columns = row.columns();
    let mut result: Vec<Value> = Vec::with_capacity(columns.len());
    for c in columns {
        let value = row.try_get_raw(c.ordinal()).unwrap();
        let value = SerPgValueRef(value);
        let value = serde_json::to_value(&value).unwrap();
        result.push(value);
    }
    result
}

/// Can be used with serialize_with
pub fn serialize_pg_value_ref<S>(value: &PgValueRef, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if value.is_null() {
        return s.serialize_none();
    }
    let value = value.clone();
    let info = value.type_info();
    let name = info.name();
    match name {
        "BOOL" => {
            let v: bool = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_bool(v)
        }
        "INT2" => {
            let v: i16 = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_i16(v)
        }
        "INT4" => {
            let v: i32 = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_i32(v)
        }
        "INT8" => {
            let v: i64 = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_i64(v)
        }
        "FLOAT4" => {
            let v: f32 = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_f32(v)
        }
        "FLOAT8" => {
            let v: f64 = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_f64(v)
        }
        #[cfg(feature = "decimal")]
        "NUMERIC" => {
            let v: sqlx::types::Decimal = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_str(&v.to_string())
        }
        "CHAR" | "VARCHAR" | "TEXT" | "\"CHAR\"" => {
            let v: String = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_str(&v)
        }
        "BYTEA" => {
            let v: Vec<u8> = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_some(&v)
        }
        "JSON" | "JSONB" => {
            let v: Value = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_some(&v)
        }
        #[cfg(feature = "chrono")]
        "TIMESTAMP" => {
            let v: chrono::NaiveDateTime = Decode::<Postgres>::decode(value).unwrap();
            let v = v.format("%Y-%m-%dT%H:%M:%S.%f").to_string();
            s.serialize_str(&v)
        }
        #[cfg(feature = "chrono")]
        "TIMESTAMPTZ" => {
            let v: chrono::DateTime<chrono::Utc> = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_str(&v.to_rfc3339())
        }
        #[cfg(feature = "chrono")]
        "DATE" => {
            let v: chrono::NaiveDate = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_str(&v.to_string())
        }
        #[cfg(feature = "chrono")]
        "TIME" => {
            let v: chrono::NaiveTime = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_str(&v.to_string())
        }
        #[cfg(feature = "uuid")]
        "UUID" => {
            let v: sqlx::types::Uuid = Decode::<Postgres>::decode(value).unwrap();
            let v = v.to_string();
            s.serialize_str(&v)
        }
        "geometry" => {
            let v: Vec<u8> = Decode::<Postgres>::decode(value).unwrap();
            s.serialize_some(&v)
        }
        _ => {
            let error_message = format!("Failed to deserialize postgres type {} as string", name);
            let v: String = Decode::<Postgres>::decode(value).expect(&error_message);
            s.serialize_str(&v)
        }
    }
}

/// Can be used with serialize_with
pub fn serialize_pgrow_as_vec<S>(x: &PgRow, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let cols = x.columns();
    let mut seq = s.serialize_seq(Some(cols.len()))?;
    for c in cols {
        let c: PgValueRef = x.try_get_raw(c.ordinal()).unwrap();
        let c = SerPgValueRef(c);
        seq.serialize_element(&c)?;
    }
    seq.end()
}

/// Can be used with serialize_with
pub fn serialize_pgrow_as_map<S>(x: &PgRow, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let cols = x.columns();
    let mut map = s.serialize_map(Some(cols.len()))?;
    for col in cols {
        let c: PgValueRef = x.try_get_raw(col.ordinal()).unwrap();
        let c = SerPgValueRef(c);
        map.serialize_entry(col.name(), &c)?;
    }
    map.end()
}

/// SerVecPgRow::from(pg_row) will make your row serialize as a vector.
#[derive(Serialize)]
pub struct SerVecPgRow(#[serde(serialize_with = "serialize_pgrow_as_vec")] PgRow);

/// SerMapPgRow::from(pg_row) will make your row serialize as a map.
/// If you have multiple columns with the same name, the last one will win.
#[derive(Serialize)]
pub struct SerMapPgRow(#[serde(serialize_with = "serialize_pgrow_as_map")] PgRow);

impl From<PgRow> for SerMapPgRow {
    fn from(row: PgRow) -> Self {
        SerMapPgRow(row)
    }
}

impl std::ops::Deref for SerMapPgRow {
    type Target = PgRow;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SerMapPgRow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<SerMapPgRow> for PgRow {
    fn from(value: SerMapPgRow) -> Self {
        value.0
    }
}

/// SerPgValueRef::from(pg_value_ref) will make your value serialize as its closest serde type.
#[derive(Serialize)]
pub struct SerPgValueRef<'r>(#[serde(serialize_with = "serialize_pg_value_ref")] PgValueRef<'r>);

impl From<PgRow> for SerVecPgRow {
    fn from(row: PgRow) -> Self {
        SerVecPgRow(row)
    }
}

impl std::ops::Deref for SerVecPgRow {
    type Target = PgRow;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SerVecPgRow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<SerVecPgRow> for PgRow {
    fn from(value: SerVecPgRow) -> Self {
        value.0
    }
}

impl From<SerMapPgRow> for Value {
    fn from(value: SerMapPgRow) -> Self {
        serde_json::to_value(value).unwrap()
    }
}

impl From<SerVecPgRow> for Value {
    fn from(value: SerVecPgRow) -> Self {
        serde_json::to_value(value).unwrap()
    }
}
