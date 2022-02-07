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