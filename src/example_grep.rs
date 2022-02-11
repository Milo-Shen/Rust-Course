use std::env::args;
use std::{fs, process};

pub fn learning_grep_example() {
    println!("Start to learn grep example");
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // 此处是返回 Config 所以可以把原先的 parse_config 函数改编成如下的 new 关联函数的形式
    // args 是一个 Vec 的切片
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // 异常处理
        if args.len() < 3 {
            return Err("not enough arguments !");
        }
        // todo: 如果使用 arg[1] 而不是 &arg[1], 会报错: error[E0507]: cannot move out of index of `Vec<String>`
        // todo: 思考为什么，顺带不用 clone 的话，会提示 can not move
        let query = args[1].clone();
        let mut filename = String::from("./assets/");
        filename.push_str(&args[2]);
        let _filename = filename;
        return Ok(Config {
            query,
            filename: _filename,
        });
    }
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
    println!("Search for {}, in file: {}", config.query, config.filename);
    let contents = fs::read_to_string(config.filename).expect("Something went wrong when reading the file");
    //  cargo run Nobody poem.txt
    println!("With text: \n{}", contents);
}