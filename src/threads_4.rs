pub fn learning_threads() {
    println!("Start to learn threads 4");

    // 通过 Send 和 Sync Trait 来扩展并发
    // Send 和 Sync trait
    // Rust 语言的并发特性较少, 目前讲的并发特性都来自标准库 ( 而不是语言本身 )
    // 无需局限于标准库的并发, 可以自己实现并发
    // 但在 Rust 语言中有两个并发概念:
    // std::marker::Sync 和 std::marker::Send 这两个 Trait

    // Send: 允许线程间转移所有权
    // 实现 Send trait 的类型可在线程间转移所有权
    // Rust 中几乎所有的类型都实现了 Send
    //  - 但 Rc<T> 没有实现 Send, 它只用于单线程情景
    // 任何完全由 Send 类型组成的类型也被标记为 Send
    // 除了原始指针以外, 几乎所有的基础类型都是 Send

    // Sync: 允许从多线程访问
    // 实现 Sync 的类型可以安全的被多个线程引用
    // 也就是说: 如果 T 是 Sync, 那么 &T 就是 Send
    //  - 引用可以被安全的送往另一个线程
    // 基础类型都是 Sync
    // 完全由 Sync 类型组成的类型也是 Sync
    //  - 但, Rc<T> 不是 Sync 的
    //  - RefCell<T> 和 Cell<T> 家族也不是 Sync 的
    //  - 而, Mutex<T> 是 Sync 的

    // 手动实现 Send 和 Sync 是不安全的
}