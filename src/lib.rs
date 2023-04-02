use std::{fs, env, process};
use std::error::Error;

fn print_outside() {
    println!("print_outside called");
}

pub mod father_mod {
    fn print_father_mod() {
        println!("print_father_mod called");
    }

    pub mod son_mod {
        pub fn print_son_mod() {
            // super 关键字
            // super: 用来访问父级模块路径中的内容, 类似文件系统中的 ..
            super::super::print_outside();
            super::print_father_mod();
            println!("print_son_mod")
        }
    }
}

mod front_of_house;

mod back_of_house {
    // pub 放在 struct 前, struct 是公共的，而 struct 的字段默认是私有的
    #[derive(Debug)]
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
    println!("meal: {:?}", meal);
    println!("I'd like {} toast please", meal.toast);
    // 在外部不能访问私有变量, 即使是只读的也不行
    // error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
    // println!("I'd like {} seasonal fruit please", meal.seasonal_fruit);

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

// 可以使用  CASE_INSENSITIVE=0 cargo run Too poem.txt 来测试
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

// Rust 面向对象编程
#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0 as f64,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                return Some(value);
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// 实现面向对象的设计模式
pub trait State {
    // Box<Self> 与 self 的区别: 这个语法意味着这个方法，它只能被包裹着当前类型的 Box 实例所调用
    // 它会在调用过程中获取 Box<Self> 的所有权, 并使旧的状态实失效, 从而将 Post 的状态值，转化为一个新的状态
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str { "" }
}

pub struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        todo!()
    }
}

pub struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

pub struct DraftPost {
    content: String,
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // as_ref: Converts from &Option<T> to Option<&T>.
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        // take 的作用: takes the value out of the option, leaving a None in its place.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
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

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}

// 将错误信息写入到标准错误而不是标准输出
// 标准输出 vs 标准错误
// 标准输出: stdout
//  - println!
// 标准错误: stderr
//  - eprintln!


// learning pub use
pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Purple
    }
}

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

