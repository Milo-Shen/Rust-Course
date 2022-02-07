pub fn learning_tests() {
    println!("Start to learn test");

    // 测试函数体 ( 通常 ) 执行的 3 个操作
    //  - 准备数据 / 状态
    //  - 运行被测试的代码
    //  - 断言 ( Assert ) 结果

    // 解刨测试函数
    // 测试函数需要使用 test 属性 ( attribute ) 进行标注
    //  - attribute 就是一段 Rust 代码的元数据
    //  - 在函数上加上 #[test]，可把函数变成测试函数

    // 运行测试
    // 使用 cargo test 命令运行所有测试函数
    //  - Rust 会构建一个 Test Runner 可执行文件
    //    其会运行标注了 test 的函数，并报告其运行是否成功

    // 当使用 cargo 创建 library 项目的时候，会生成一个 test module，里面有一个 test 函数
    //  - 你可以添加任意数量的 test module 或函数
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        return self.length > other.length && self.width > other.width;
    }
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
    // failures: learning_test::it_works
    // panic!("make the test fail");
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 7,
        length: 8,
    };
    let smaller = Rectangle {
        width: 1,
        length: 1,
    };
    assert!(larger.can_hold(&smaller));
}

// 使用 assert_eq! 和 assert_ne! 测试相等性
// 都来自标准库，分别用于判断两个参数是否 (assert_eq!)相等 或 (assert_ne!)不等
// 上述两个宏内部使用的就是 -- 和 != 运算符
// 当断言失败时：自动打印出两个参数的值
//  - 使用 debug 格式打印参数
//     要求参数实现了 PartialEq 和 Debug Traits ( 所有的基本类型和标准库里大部分类型都实现了 )

fn add_two(a: i32) -> i32 { a + 2 }

fn add_three(a: i32) -> i32 { a + 3 }

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
    // 参数反过来写也是可以的，如下:
    assert_eq!(add_two(2), 4);
    assert_ne!(add_three(2), 6);
}

// 添加自定义的错误信息
// 可以向 assert!、assert_eq!、assert_ne! 添加可选的自定义消息
//  - 这些自定义消息和失败消息都会被打印出来
//  - assert! : 第一个参数必填，自定义参数作为第二个参数
//  - assert_eq! 和 assert_ne! : 前两个参数必填，自定义消息作为第三个参数
//  - 自定义消息参数会被传递给 format! 宏，可以使用 {} 占位符
fn greeting(name: &str) -> String {
    return format!("Hello {} !", name);
}

#[test]
fn greeting_contains_name() {
    let result = greeting("carol");
    assert!(
        result.contains("carol"),
        "自定义信息: 找不到 carol，值是: {}",
        result
    );
}

// 使用 should_panic 检查恐慌
// 测试处了验证代码的返回值是否正确，还需要验证代码是否如预期的处理了发生错误的情况
// 可验证代码在特定情况下是否发生了 panic
// should_panic 属性 ( attribute ):
//  - 函数 panic: 测试通过
//  - 函数没有 panic: 测试失败

struct Guess {
    _value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        return Guess { _value: value };
    }
}

#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}

// 让 should_panic 更精确
// 为 should_panic 属性添加一个可选的 expected 参数，将检查失败消息中是否包含所指定的文字
// expected 后面的内容只要被 panic! 的信息包含即可，不必完全相同
#[test]
#[should_panic(expected = "Guess value must be between 1 and 100, got 200")]
fn greater_than_100_expected() {
    Guess::new(200);
}

// panic message: `"Guess value must be between 1 and 100, got 200"`,
// expected substring: `"Guess value must be between 1 and 100, got 201"`
// 需要特别注意的是: 若是指定了 expected，但是 expected 的信息未被包含在 panic! message 中，那么即使 panic 发生了，也无法通过测试
#[test]
#[should_panic(expected = "Guess value must be between 1 and 100, got 201")]
fn greater_than_100_expected_1() {
    Guess::new(200);
}