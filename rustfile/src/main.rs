use std::fs;
use std::path::Path;
fn main() {

    let path = Path::new("output");

    if !path.exists() {
        match fs::create_dir_all(path) {
            Ok(_) => println!("文件夹已创建。"),
            Err(e) => println!("创建文件夹时出错: {}", e),
        }
    }
}