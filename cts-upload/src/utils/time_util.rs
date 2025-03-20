use chrono::{Datelike, FixedOffset, Timelike, Utc};

/// 创建时间目录
/// @param path 根目录
/// @return path 返回实际路径
pub fn create_time_dir(path: &str) -> String {
    let offset = FixedOffset::east_opt(8 * 3600).unwrap();
    // 获取当前 UTC 时间
    let utc_now = Utc::now();
    let date = utc_now.with_timezone(&offset);
    let time = format!(
        "{}/{}/{}/{}/{}/{}",
        path,
        date.year(),
        date.month(),
        date.day(),
        date.hour(),
        date.minute()
    );
    time
}
