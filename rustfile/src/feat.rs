use std::fs;
use std::path::Path;

fn main() {
    // 指定文件夹路径
    let path = Path::new("my_directory");

    // 检查文件夹是否存在
    if path.exists() {
        println!("文件夹已存在。");
    } else {
        // 创建文件夹
        match fs::create_dir_all(path) {
            Ok(_) => println!("文件夹已创建。"),
            Err(e) => println!("创建文件夹时出错: {}", e),
        }
    }
}