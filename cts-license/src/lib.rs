pub mod license;

fn add_newlines(s: &str, len: usize) -> String {
    s.chars()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            // 每隔两个字符添加一个回车
            if i % len == 0 && i != 0 {
                acc.push('\n');
            }
            acc.push(c);
            acc
        })
}


#[cfg(test)]
mod tests {
    use crate::license::rsa::License;
    use super::*;
    #[test]
    fn test_license() {
        License::check_license("./license.lic", "FeatureServer").expect("Valid");
    }
}
