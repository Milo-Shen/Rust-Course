use std::env::args;
use std::fs;

pub fn learning_grep_example() {
    println!("Start to learn grep example");
}

// args 是一个 Vec 的切片
fn parse_config(args: &[String]) -> (&str, String) {
    // todo: 如果使用 arg[1] 而不是 &arg[1], 会报错: error[E0507]: cannot move out of index of `Vec<String>`
    // todo: 思考为什么
    let query = &args[1];
    let mut filename = String::from("./assets/");
    filename.push_str(&args[2]);
    let _filename = filename;
    // todo: as_str 输出的是一个拷贝还是引用 ?
    return (query, _filename);
}

// 二进制程序关注点分离的指导性原则
//  - 将程序拆分成 main.rs 和 lib.rs，将业务逻辑放到 lib.rs
//  - 当命令行解析逻辑较少时，将它放在 main.rs 也行
//  - 当命令行解析逻辑复杂时，需要将它从 main.rs 提取到 lib.rs
pub fn my_grep() {
    // args().collect() 若是接收到非法 unicode 字符会发生恐慌
    let args: Vec<String> = args().collect();
    // 若是我们运行: cargo run a b 会输出: ["target/debug/rust_course", "a", "b"]
    println!("{:?}", args);
    let (query, filename) = parse_config(&args);
    println!("Search for {}, in file: {}", query, filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong when reading the file");
    //  cargo run Nobody poem.txt
    println!("With text: \n{}", contents);
}