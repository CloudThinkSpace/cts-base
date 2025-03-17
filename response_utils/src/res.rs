use std::fmt::Debug;

use axum::{
    body::Body,
    http::{header, HeaderValue},
    response::{IntoResponse, Response},
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ResResult<T> {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

/// 实现intoResponse接口
impl<T> IntoResponse for ResResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let data = Self {
            code: self.code,
            data: self.data,
            msg: self.msg,
        };
        // 序列号数据
        let json_string = match serde_json::to_string(&data) {
            Ok(v) => v,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )
                    .body(Body::from(e.to_string()))
                    .unwrap();
            }
        };

        json_string.into_response()
    }
}

impl<T> ResResult<T>
where
    T: Serialize,
{
    /// 创建成功结果对象
    /// @param data参数
    pub fn with_success(data: T) -> Response {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
            Self {
                code: 200,
                data: Some(data),
                msg: Some("success".to_string()),
            },
        )
            .into_response()
    }

    /// 创建错误对象
    /// @param err 参数
    pub fn with_error(err: &str) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
            Self {
                code: 500,
                data: None,
                msg: Some(err.to_string()),
            },
        )
            .into_response()
    }

    /// 创建错误对象
    /// @param err 错误信息
    /// @param code 错误编码
    /// @param status_code 错误状态
    pub fn with_error_code(err: &str, code: i32, status_code: StatusCode) -> Response {
        (
            status_code,
            [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
            Self {
                code,
                data: None,
                msg: Some(err.to_string()),
            },
        )
            .into_response()
    }

    /// 创建成功对象
    /// @param data 数据参数
    /// @param msg 成功信息参数
    pub fn with_success_msg(data: T, msg: &str) -> Response {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
            Self {
                code: 200,
                data: Some(data),
                msg: Some(msg.to_string()),
            },
        )
            .into_response()
    }
}
