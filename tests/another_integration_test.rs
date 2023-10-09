use rust_course;

// 运行指定的集成测试
//  - 运行一个特定的集成测试: cargo test 函数名
//  - 运行某个测试文件内的所有测试: cargo test --test 文件名
//    - 例子: cargo test --test another_integration_test
#[test]
fn it_really_adds_two() {
    assert_eq!(5, rust_course::add_two(3));
}
