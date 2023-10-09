// 通过这种子目录的方式来命名, Rust 就不会把当前模块视作为一个集成测试文件了，在测试的输出中，也不会出现 common 相关的区域了
// 因为 tests 下面的子目录不会被视为单独的 crate 进行编译，更不会测试的输出中拥有自己的区域，因为此时的 common 只是一个普通的模块而已
pub fn setup() {
    println!("This is the common utils for integration test");
}
