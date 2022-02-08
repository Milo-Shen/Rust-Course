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

// 测试私有函数
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// 测试的分类
// Rust 对测试的分类:
//  - 单元测试
//  - 集成测试
// 单元测试:
//  - 小、专注
//  - 一次对一个模块进行隔离的测试
//  - 课测试 private 接口
// 集成测试:
//  - 在库外部，和其他外部代码一样使用你的代码
//  - 只能使用 public 接口
//  - 可能在每个测试中使用到多个模块

// 单元测试 - 通过 #[cfg(test)]
// 我们通常一般把单元测试和被测试的代码都放在 src 目录下的同一个文件中，同时每个源代码文件都要建立一个 tests 模块来放置测试函数，并使用 #[cfg(test)] 来对测试模块进行标注
// tests 模块上的 #[cfg(test)] 标注:
//  - 只有运行 cargo test 才编译和运行代码
//  - 运行 cargo build 则不会
// 集成测试在不同的目录，它不需要 #[cfg(test)] 标注
// cfg: configuration ( 配置 )
//  - 告诉 Rust 下面的条目只有在指定的配置选项下才会被包含
//  - 配置选项 test: 由 Rust 提供，用来编译和运行测试
//    - 只有 cargo test 才会编译代码，包括模块中的 helper 函数和 #[test] 标注的函数
// todo: 测试一下 cargo build 的时候是不是真的不会编译下面代码，那么 cargo run 的时候呢 ？
#[cfg(test)]
mod tests {
    #[derive(Debug)]
    pub struct Rectangle {
        length: u32,
        width: u32,
    }

    // 因为下面的代码也在 #[cfg(test)] 所标注的 mod tests 下，所以这段代码在 cargo test 的时候也会被编译
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
        // 若是运行 cargo test -- -- show-output 则会输出: Use expected syntax, message is: Guess value
        // 若是运行 cargo test，则只会在 unit test fail 的时候显示 println! 内的信息
        println!("Use expected syntax, message is: Guess value must be between 1 and 100, got 200");
        Guess::new(200);
    }

    // panic message: `"Guess value must be between 1 and 100, got 200"`,
    // expected substring: `"Guess value must be between 1 and 100, got 201"`
    // 需要特别注意的是: 若是指定了 expected，但是 expected 的信息未被包含在 panic! message 中，那么即使 panic 发生了，也无法通过测试
    // #[test]
    // #[should_panic(expected = "Guess value must be between 1 and 100, got 201")]
    // fn greater_than_100_expected_1() {
    //     Guess::new(200);
    // }

    // 在测试中使用 Result<T,E>
    // 无需 panic, 可使用 Result<T,E> 作为返回类型编写测试
    // 需要注意的是: 不要在使用 Result<T,E> 编写的测试上标注 #[should_panic]
    // 因为这种情况下不会产生 panic，因为会走 Err
    #[test]
    fn it_works_1() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // 控制测试如何运行
    // 改变 cargo test 的行为: 添加命令行参数
    // 默认行为:
    //  - 并行运行
    //  - 所有测试
    //  - 捕获 ( 不显示 ) 所有输出，使读取与测试结果相关的输出更容易
    // 命令行参数:
    //  - 针对 cargo test 的参数: 紧跟 cargo test 后
    //  - 针对 测试可执行程序: 放在 -- 之后
    // cargo test --help
    // cargo test -- --help ( 这个命令有一个编译的过程 )

    // 并行/连续运行测试

    // 并行运行测试
    // 运行多个测试: 默认使用多个线程并行运行
    //  - 运行块
    // 确保测试之间:
    //  - 不会相互依赖
    //  - 不依赖于某个共享状态 ( 环境，工作目录、环境变量等等 )
    // 如何控制运行测试时并行的数量 ?

    // 可以使用 --test-threads 参数
    //  - 这个参数是传递给二进制文件的
    //  - 不想以并行方式运行测试，或相对线程数进行细粒度控制
    //  - 可以使用 --test-threads 参数，后面跟着线程的数量
    //  - 例如: cargo test -- --test-threads=1

    // 显式函数输出
    // 默认, 如果通过测试，Rust 的 test 库会捕获所有打印到标准输出的内容
    // 例如, 如果被测试代码中用到了 println! :
    //  - 如果测试通过: 不会在终端看到 println! 打印的内容
    //  - 如果测试失败: 会看到 println! 打印的内容 和 失败信息

    // 如果我们想在成功的测试中看到打印的内容 : --show-output
    // 例子: cargo test -- --show-output

    // 按名称运行测试的子集
    // 选择运行的测试: 将测试的名称 ( 一个或多个 ) 作为 cargo test 的参数
    // 运行单个测试: 指定测试名，例如: cargo test it_works_1
    // 运行多个测试: 指定测试名的一部分 ( 模块名也可以 )，例如: cargo test it_works

    // 忽略某些测试，运行剩余的测试
    // 可以使用 ignore 属性 ( attribute )
    // 运行被忽略 ( ignore ) 的测试: cargo test -- --ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(1, 1);
    }
}

// 运行多个测试: 指定测试名的一部分 ( 模块名也可以 )，例如: cargo test it_works_another
// 在这个例子中，我们可以指定只运行某个模块中的测试，例如: cargo test another_tests
#[cfg(test)]
mod another_tests {
    // 下面两种导入方式都可以，这个例子中，我们使用 super 关键字导入
    // use crate::learning_test::internal_adder;
    use super::internal_adder;

    #[test]
    fn it_works_another() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_private_functions() {
        // 下面的函数 internal_adder 是私有函数，但是仍旧可以被 rust 进行测试
        assert_eq!(internal_adder(1, 2), 3);
    }
}
