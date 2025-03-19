use chrono::{Datelike, Local, Timelike};

/// 创建时间目录
/// @param path 根目录
/// @return path 返回实际路径
pub fn create_time_dir(path: &str) -> String {
    let date = Local::now();
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