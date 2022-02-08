use rust_course;

// 这里不需要加 #[cfg(test)] 这个申明了，因为 rust 会对 tests 目录进行特殊处理
// 只有当运行 cargo test 的时候才会进行编译
#[test]
fn it_add_two() {
    assert_eq!(4, rust_course::add_two(2))
}