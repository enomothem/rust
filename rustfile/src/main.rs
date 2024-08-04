// ---------------------------------------------------------------
use std::fs;
use std::path::Path;
use std::io::{self, Write};

// fs::create_dir v
// fs::create_dir_all v     简而言之，create_dir 只负责创建单个目录，而 create_dir_all 负责创建整个路径上所有不存在的目录。
// fs::File::create v
// fs::create_file x

// ---------------------------------------------------------------
use std::thread;
use std::time::Duration;


// ---------------------------------------------------------------
use colored::Colorize;




fn main() {
    
    // ---------------------------------------------------------------
    // File Control 
    // ---------------------------------------------------------------

    let folder: &str = "output";

    // let _file = format!("{}{}", folder, "/test.txt");
    let file_path = Path::new(folder).join("test.txt");

    // match 表达式：match 提供了更全面的方式来处理枚举值，包括 Option 和 Result。它允许你为不同的模式（例如 Some 和 None）提供不同的代码分支。
    // match 更适合于你需要对所有可能的值进行处理的情况。
    
    // if !file_path.exists() {
    //     match fs::create_dir_all(file_path) {
    //         Ok(_) => println!("{} Folder:{} created successfully!", "[+]".green(), folder.cyan()),
    //         Err(e) => println!("Folder created error: {}", e),
    //     }
    // }

    // Some 表达式：当你想要直接使用 Option 类型中的值时，可以使用 if let 语句来检查它是否是 
    // Some 并解构它。这种方式简洁且直接，适用于你只关心 Some 值的情况。

    // 检查文件夹是否存在，如果不存在则创建
    if let Some(parent) = Path::new(&file_path).parent() {
        if !parent.exists() {
            fs::create_dir(parent).expect("create folder err");
            println!("{} Folder:{} created successfully!", "[+]".green(), folder.cyan());
        }
    }

    // 检查文件是否存在，如果不存在则创建
    if !Path::new(&file_path).exists() {
        fs::File::create(&file_path).expect("create file err");
    }

    // 延时3秒
    println!("{} waiting 3 seconds", "[*]".blue());
    io::stdout().flush().unwrap(); // 确保打印到控制台
    print!("[");
    for _ in 0..3 {
        thread::sleep(Duration::from_secs(1));
        print!("=======================");
        io::stdout().flush().unwrap(); // 刷新输出缓冲区
    }
    print!("] %100\n");
    

    // delete

    if Path::new(&file_path).exists() {
        fs::remove_dir_all(folder).expect("create file err");
        println!("{} Folder:{} deleted successfully!", "[-]".red(), folder.cyan());
    }
}