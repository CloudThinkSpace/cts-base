use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, HeaderValue};

/// ### 创建文件类型headerMap
/// - `@param`: file_name 文件名
/// - `@param`: content_type 内容类型
/// - `@return`: 返回HeaderMap 类型
fn create_file_header_map(file_name: String, content_type: ContentType) -> HeaderMap {
    // 查找是否有点符号
    let index = file_name.find('.').unwrap_or(usize::MAX);
    //文件扩展名
    let mut ext_name = None;
    if index != usize::MAX {
        ext_name = Some(&file_name[index + 1..]);
    }
    let content_type = match content_type {
        ContentType::Image => {
            format!("image/{}", ext_name.unwrap())
        }
        ContentType::Video => "video/*".to_string(),
        ContentType::Other => "application/octet-stream".to_string(),
    };
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str(&content_type).unwrap(),
    );
    headers
}

pub enum ContentType {
    Image,
    Video,
    Other,
}
