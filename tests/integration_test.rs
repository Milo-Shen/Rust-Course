// 集成测试
// 在 Rust 里，集成测试完全位于被测试库的外部
// 目的: 是测试被测试库的多个部分是否能正确一起工作
// 集成测试的覆盖率很重要

// tests 目录
// 创建集成测试: tests目录，这个目录和 src 目录并列 ( cargo 会自动在这个目录下寻找集成测试文件，可以创建任意多的测试文件 )
// tests 目录下的每个测试文件都是单独的一个 crate
//  - 需要将被测试库导入
// 无需标注 #[cfg(test)], tests 目录会被特殊对待
//  - 只有 cargo test, 才会编译 tests 目录下的文件

use rust_course;

mod common;

// 这里不需要加 #[cfg(test)] 这个申明了，因为 rust 会对 tests 目录进行特殊处理
// 只有当运行 cargo test 的时候才会进行编译
#[test]
fn integration_test_add_two() {
    // 通过这种子目录的方式来命名, Rust 就不会把当前模块视作为一个集成测试文件了，在测试的输出中，也不会出现 common 相关的区域了
    // 因为 tests 下面的子目录不会被视为单独的 crate 进行编译，更不会测试的输出中拥有自己的区域，因为此时的 common 只是一个普通的模块而已
    common::setup();
    assert_eq!(4, rust_course::add_two(2));
}

// 运行指定的集成测试
//  - 运行一个特定的集成测试: cargo test 函数名
//  - 运行某个测试文件内的所有测试: cargo test --test 文件名
//    - 例子: cargo test --test another_integration_test

// 集成测试中的子模块
// tests 目录下的每个文件都会被编译成单独的 crate
//  - 这些文件不共享行为 ( 与 src 下的文件规则不同 )

// 针对 binary crate 的集成测试
// 如果项目是 binary crate，只含有 src/main.rs 没有 src/libs;
//  - 不能在 tests 目录下创建集成测试
//  - 无法把 main.rs 的函数导入作用域
// 只有 library crate 才能暴露函数给其他 crate 用
// binary crate 意味着独立运行