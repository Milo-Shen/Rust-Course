use std::fs::File;
use std::io::{self, ErrorKind, Read};

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

    // use unwrap_or_else to simplify the code
    let f = File::open("./assets/Jack.txt").unwrap_or_else(|error| {
        // todo: 研究这里的这个 enum 为什么可以用 == 来判断相等 ?
        if error.kind() == ErrorKind::NotFound {
            File::create("./assets/Jack.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Error opening the file: {:?}", error)
        }
    });

    // use unwrap method
    // 如果 Result 的结果是 Ok, 返回 Ok 里面的值
    // 如果 Result 的结果是 Err，调用 panic! 宏，缺点是不能自定义错误信息
    let f = File::open("./assets/hello.txt").unwrap();

    // use expect syntax can specify error message
    // thread 'main' panicked at 'Can not open the file: Os
    // let f = File::open("./assets/not_exist.txt").expect("Can not open the file");

    // spread errors to function caller
    fn read_username_from_file() -> Result<String, io::Error> {
        let file = File::open("./assets/hello.txt");
        let mut username = match file {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        return match username.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        };
    }

    let username = read_username_from_file().unwrap();
    println!("The username from file is: {}", username);

    // use ? syntax to spread error
    fn read_username_from_file_question_mark() -> Result<String, io::Error> {
        // 此处使用 mut file 是因为 read_to_string 的签名为:
        // fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize>
        let mut file = File::open("./assets/hello.txt")?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        return Ok(s);
    }

    let username = read_username_from_file_question_mark().unwrap();
    println!("The username from file is: {}", username);
}