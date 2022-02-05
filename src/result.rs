use std::fs::File;
use std::io::ErrorKind;

pub fn learning_result() {
    println!("Start to learn Result");

    // Result 的定义
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let f = File::open("./assets/hello.txt");

    // 此处的 f 的所有权发生了转移，所以后面无法继续使用 f 了
    match f {
        Result::Ok(data) => println!("open file success"),
        Result::Err(err) => println!("open file failed"),
    };

    let f = File::open("./assets/hello.txt");
    let file = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("./assets/hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            other => panic!("Error opening the file: {:?}", other),
        },
    };
}