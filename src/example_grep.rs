use std::{fs, process};
use std::env::args;
use rust_course::{Config, run};

pub fn learning_grep_example() {
    println!("Start to learn grep example");
}

// 二进制程序关注点分离的指导性原则
//  - 将程序拆分成 main.rs 和 lib.rs，将业务逻辑放到 lib.rs
//  - 当命令行解析逻辑较少时，将它放在 main.rs 也行
//  - 当命令行解析逻辑复杂时，需要将它从 main.rs 提取到 lib.rs

// 经过拆分后，留在 my_grep 函数中的功能有:
// 使用参数调用命令行解析逻辑
// 使用其他配置
// 调用 lib.rs 中的 run 函数
// 处理 run 函数可能出现的错误
pub fn my_grep() {
    // args().collect() 若是接收到非法 unicode 字符会发生恐慌
    let args: Vec<String> = args().collect();
    // 若是我们运行: cargo run a b 会输出: ["target/debug/rust_course", "a", "b"]
    println!("{:?}", args);
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}