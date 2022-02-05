use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::slice::RSplit;

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

    // use ? syntax to spread error ( only used Result )
    // ？运算符是传播错误的一种快捷方式，只能用于返回 Result 的函数：
    // 1. 如果 Result 的值是 Ok, Ok 中的值就是表达式的结果，然后继续执行程序
    // 2. 如果 Result 的值是 Err, Err 就是整个函数的返回值，就像是使用了 return
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

    // ? 与 from 函数
    // Trait std:convert:From 上的 from 函数，被 ? 所应用的错误，会隐式得被 from 函数处理
    // 他所接收的错误类型会被转化为当前函数返回类型所定义的错误类型
    // 用途: 针对不同的错误原因，返回同一种错误类型 (只要每个错误类型实现了转化为所返回错误类型的 from 函数)

    // use chain method
    fn read_username_from_file_chain_method() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("./assets/hello.txt")?.read_to_string(&mut s)?;
        return Ok(s);
    }
    let username = read_username_from_file_chain_method().unwrap();
    println!("The username from file is: {}", username);

    // ? can only be used for functions that return type Result
    // ? & main

    // fn main() -> Result<(), Box<dyn Error>> {
    //     let f = File::open("./assets/hello.txt")?;
    //     Ok(());
    // }
}