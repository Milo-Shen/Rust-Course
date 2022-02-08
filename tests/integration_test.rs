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

// 这里不需要加 #[cfg(test)] 这个申明了，因为 rust 会对 tests 目录进行特殊处理
// 只有当运行 cargo test 的时候才会进行编译
#[test]
fn it_add_two() {
    assert_eq!(4, rust_course::add_two(2))
}