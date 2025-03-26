use crate::add_newlines;
use crate::license::{Error, PRIV_KEY, PUB_KEY};
use chrono::{DateTime, Days, Local, TimeZone};
use log::{error, info};
use rsa::pkcs1::{
    DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey,
};
use rsa::pkcs8::LineEnding;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use std::fs;

/// 许可对象
/// @param name 许可名称
/// @param expire 过期时间
#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    server: String,
    name: String,
    expire: i64,
}

impl License {
    /// 创建许可对象，默认过期时间一个月
    /// @param server 服务名称
    /// @param name 许可名称
    pub fn new(server: &str, name: &str) -> Self {
        Self::new_with_expire(server, name, 30)
    }

    /// 创建许可对象
    /// @param server 服务名称
    /// @param name 许可名称
    /// @param expire_days 过期时间，单位天
    pub fn new_with_expire(server: &str, name: &str, expire_days: u64) -> Self {
        // 获取当前时间
        let now = Local::now();
        // 添加指定天数
        let expire = now.checked_add_days(Days::new(expire_days)).unwrap();

        Self {
            server: server.to_string(),
            name: name.to_string(),
            expire: expire.timestamp_millis(),
        }
    }

    /// 创建许可字符串
    pub fn generate_license(&self) -> Result<String, Error> {
        // 随机对象
        let mut rng = rand::thread_rng();
        // 加密对象
        let pub_key = RsaPublicKey::from_pkcs1_pem(PUB_KEY)
            .map_err(|_| Error::PublicKeyCreateError("秘钥解析失败".to_string()))?;
        // 解析成字符串
        let license_info = serde_json::to_string(&self)
            .map_err(|_| Error::LicenceCreateError("秘钥解析失败".to_string()))?;
        // 加密数据
        let enc_data = pub_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, license_info.as_bytes())
            .map_err(|_| Error::PublicKeyEncryptError("许可加密失败".to_string()))?;
        // hex 编码
        let license_hex = hex::encode(&enc_data);
        // 每隔64个字符串添加一个换行符
        let license = add_newlines(&license_hex, 64);
        Ok(license)
    }

    /// 校验许可,如果失败抛出错误信息
    pub fn check_license(path: &str, server: &str) -> Result<(), Error> {
        // 读取许可文件
        let license = read_license(path).map_err(Error::LicenceInvalid)?;
        // 创建解析对象
        let priv_key = RsaPrivateKey::from_pkcs1_pem(PRIV_KEY)
            .map_err(|_| Error::PrivateCreateError("秘钥解析失败".to_string()))?;
        // hex 解码
        let data = hex::decode(license)
            .map_err(|_| Error::HexDecodeError("解码字符串错误".to_string()))?;
        // 解密数据
        let dec_data = priv_key
            .decrypt(Pkcs1v15Encrypt, &data)
            .map_err(|_| Error::PrivateDecryptError("许可解密失败".to_string()))?;
        // 许可json字符串
        let licence_key = String::from_utf8(dec_data)
            .map_err(|_| Error::LicenceCreateError("许可解析错误".to_string()))?;
        // 解析成对象
        let license: License = serde_json::from_str(&licence_key)
            .map_err(|_| Error::LicenceCreateError("许可失效".to_string()))?;
        // 当前时间
        let current_time = Local::now().timestamp_millis();
        // 判断是否过期,并且名称是否一致
        match license.expire > current_time && server == license.server {
            true => {
                let logo_info = logo();
                // 时间戳转时间
                let date_time: DateTime<Local> = DateTime::from_timestamp_millis(license.expire)
                    .unwrap()
                    .into();
                // 时间转字符串
                let date_str = date_time.format("%Y-%m-%d %H:%M:%S");
                let logo_msg = format!("{}\n     许可有效期至：{}", logo_info, date_str);
                info!("\n{}", logo_msg);
                Ok(())
            }
            false => {
                error!("\n========================================================================\n
                                                许可无效，请重新申请许可\n
                       ===========================================================================================");
                Err(Error::LicenceExpired(
                    "许可失效，请重新申请许可".to_string(),
                ))
            }
        }
    }
}

/// 创建rsa秘钥
pub fn create_key_pair() -> Result<(String, String), String> {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let pub_key = pub_key.to_pkcs1_pem(LineEnding::CRLF).unwrap();
    let priv_key = priv_key.to_pkcs1_pem(LineEnding::CRLF).unwrap().to_string();
    Ok((pub_key, priv_key))
}

/// 读取许可并简单验证
fn read_license(path: &str) -> Result<String, String> {
    let license = fs::read_to_string(path)
        .map_err(|_| "The license file does not exist or is not accessible".to_string())?;
    // 判断许可内容是否为空
    let licence = match license.is_empty() {
        true => {
            return Err("The license content is empty".to_string());
        }
        false => license,
    };
    // 去除回车字符串
    let license = licence.chars().filter(|&c| c != '\n').collect::<String>();
    Ok(license)
}

fn logo() -> String {
    r#"
     _______  ____________  ________
    (  ____ \ \___   ____/ (  _____ \
    | (    \/     ) (      | (     \/
    | (           | |      | (______
    | (           | |      (______  )
    | (           | |             ) |
    | (____/\     | |      /\_____) |
    (_______/     )_(      \________)


     CTS License loaded successfully
    "#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Months;
    #[test]
    fn test_read_licence() {
        let now = Local::now();
        // 默认过期时间1个月
        let expire = now.checked_add_months(Months::new(1)).unwrap();
        println!("{:?}", expire);
    }
}
