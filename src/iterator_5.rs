use std::{env, fs};
use std::error::Error;
use std::f32::consts::E;

pub fn learning_iterator() {
    println!("Start to learn iterator 5");

    pub struct Config {
        query: String,
        filename: String,
        case_sensitive: bool,
    }

    impl Config {
        // 此处是返回 Config 所以可以把原先的 parse_config 函数改编成如下的 new 关联函数的形式
        // args 是一个 Vec 的切片
        pub fn new(args: &[String]) -> Result<Config, &'static str> {
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
            println!("Search for {}, in file: {}", query, _filename);
            // is_err 返回 true 或 false
            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
            return Ok(Config {
                query,
                case_sensitive,
                filename: _filename,
            });
        }

        pub fn new_iterator(mut args: std::env::Args) -> Result<Config, &'static str> {
            // 异常处理
            if args.len() < 3 {
                return Err("not enough arguments !");
            }
            args.next();
            let query = match args.next() {
                Some(x) => x,
                // todo: 这里没有 return 就报错了，思考此处有没有 return 的区别
                None => return Err("Did not get a query string"),
            };
            let _filename = match args.next() {
                Some(x) => x,
                None => return Err("Did not get a filename"),
            };
            let mut filename = String::from("./assets/");
            filename.push_str(&_filename);
            let _filename = filename;
            println!("Search for {}, in file: {}", query, _filename);
            // is_err 返回 true 或 false
            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
            return Ok(Config {
                query,
                case_sensitive,
                filename: _filename,
            });
        }
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;
        //  cargo run frog poem.txt
        println!("With text: \n{}", contents);
        let result = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };
        for line in result {
            println!("search: {}", line);
        }
        Ok(())
    }

    // 当前片引用的数据是有效的时候，切片本身才是有效的
    pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        let mut result: Vec<&str> = Vec::new();
        for line in content.lines() {
            if line.contains(query) {
                result.push(line.trim());
            }
        }
        return result;
    }

    // 使用迭代器简化 search 的代码
    pub fn search_iterator<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        content.lines().filter(|line| line.contains(query)).collect()
    }

    // 可以使用 CASE_INSENSITIVE=0 cargo run Too poem.txt 来测试
    pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        let mut result: Vec<&str> = Vec::new();
        for line in content.lines() {
            if line.to_lowercase().contains(&query) {
                result.push(line.trim());
            }
        }
        return result;
    }

    // 使用迭代器简化 search_case_insensitive 的代码
    pub fn search_case_insensitive_iterator<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
        content.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).map(|x| x.trim()).collect()
    }
}