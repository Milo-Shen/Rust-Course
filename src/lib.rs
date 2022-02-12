use std::{fs, process};
use std::error::Error;

mod front_of_house;

mod back_of_house {
    // pub 放在 struct 前, struct 是公共的，而 struct 的字段默认是私有的
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // pub enum
    // 公共枚举里面的所有变体都是公共的
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            };
        }
    }
}

// 使用 pub use 以便让导入的代码可以被外部访问
// 默认 use 导入的代码对当前文件是 private 的
pub use crate::front_of_house::hosting;

// 下面是使用相对路径引入
// use front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_wait_list();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // use absolute path: hosting
    hosting::add_to_wait_list();
    // Function `private_function` is private [E0603] use 也需要遵守私有性规则
    // hosting::private_function();  private_function 是私有函数，所以无法在这里被访问
    hosting::print_house();
    hosting::test_mod::hello();

    // todo: use 的习惯用法，函数: 将函数的父级模块引入到作用域
    // todo: struct, enum，其他: 指定完整的路径 （ 指定到本身 ）
    // todo: 但是对于同名条目：指定到父级
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub struct Config {
    query: String,
    filename: String,
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
        return Ok(Config {
            query,
            filename: _filename,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    //  cargo run frog poem.txt
    for line in search(&config.query, &contents) {
        println!("search: {}", line);
    }
    println!("With text: \n{}", contents);
    Ok(())
}

// 测试驱动开发 TDD (Test-Driven Development)
// 编写一个会失败的测试，运行该测试，确保它是按照预期的原因失败
// 编写或修改足够刚好的代码，让新测试通过
// 重构刚刚添加好的代码，确保测试会始终通过
// 返回步骤1, 继续

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}