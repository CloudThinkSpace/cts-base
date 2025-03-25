use std::fs::File;
use std::io::Write;
use clap::Parser;
use cts_license::license::rsa::License;

fn main() {

    let args = Args::parse();
    // 创建许可对象
    let license = License::new_with_expire(&args.server, &args.name, args.expire);
    // 生成许可
    let license_result = license.generate_license();
    // 许可字符串
    let license_info = match license_result {
        Ok(data) => {
            data
        }
        Err(err) => {
            eprintln!("{:?}", err);
            return;
        }
    };
    // 写入文件
    let mut file = File::create("./license.lic").expect("创建许可文件失败，查看是否有写入权限"); // 创建文件或打开文件用于写入
    // 写入字符串到文件
    file.write_all(license_info.as_bytes()).expect("写入许可信息失败");

}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Server name
    #[arg(short, long)]
    server: String,

    /// User name
    #[arg(short, long)]
    name: String,

    /// Expire of times to license, default 30 days
    #[arg(short, long, default_value_t = 30)]
    expire: u64,
}