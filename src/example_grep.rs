use std::env::args;
use std::fs;

pub fn learning_grep_example() {
    println!("Start to learn grep example");
}

pub fn my_grep() {
    // args().collect() 若是接收到非法 unicode 字符会发生恐慌
    let args: Vec<String> = args().collect();
    // 若是我们运行: cargo run a b 会输出: ["target/debug/rust_course", "a", "b"]
    println!("{:?}", args);

    // todo: 如果使用 arg[1] 而不是 &arg[1], 会报错: error[E0507]: cannot move out of index of `Vec<String>`
    // todo: 思考为什么
    let query = &args[1];
    let mut filename = String::from("./assets/");
    filename.push_str(&args[2]);
    println!("Search for {}, in file: {}", query, filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong when reading the file");
    //  cargo run Nobody poem.txt
    println!("With text: \n{}", contents);
}