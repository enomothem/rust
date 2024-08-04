use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // 确保 output 目录存在
    if !Path::new("output").exists() {
        fs::create_dir_all("output")?;
    }

    // 指定要追加的文件路径
    let file_path = "output/output.txt";
    // 创建一个字符串变量，包含要追加的数据
    let data = "这是追加到文件的内容。";

    // 打开文件用于追加，如果文件不存在则创建它
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)  // 设置为追加模式
        .open(file_path)?;

    // 将数据追加到文件
    writeln!(file, "{}", data)?;

    Ok(())
}