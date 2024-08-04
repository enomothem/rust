use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // 假设这是你的总任务量
    let total_steps = 100;
    // 这是进度条的长度
    let progress_bar_length = 30;

    for i in 0..=total_steps {
        // 计算完成的百分比
        let percent_complete = (i as f64 / total_steps as f64) * 100.0;
        // 创建进度条的完整部分
        let filled_length = (progress_bar_length as f64 * percent_complete as f64 / 100.0) as usize;
        // 创建进度条的空部分
        let bar = "=".repeat(filled_length);
        let space = " ".repeat(progress_bar_length - filled_length);
        // 打印进度条和百分比
        print!(
            "\r[{}] {:.2}%", bar + &space, percent_complete
        );
        io::stdout().flush().unwrap(); // 刷新输出缓冲区

        // 模拟一些工作
        sleep(Duration::from_millis(50));
    }

    println!("\n任务完成！");
}