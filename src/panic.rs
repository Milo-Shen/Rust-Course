pub fn learning_panic() {
    println!("Start to learn panic");

    // Rust 错误的分类
    // 可恢复的错误: 例如文件未找到，可再次尝试 Result<T,E>
    // 不可恢复的错误：bug                   panic!
    // Rust 没有类似异常的机制
    // panic!("crash and burn");
    let arr = vec![1, 2, 3];
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // todo: mac os 上使用 set RUST_BACKTRACE=1 无效
    // let b = arr[100];
}