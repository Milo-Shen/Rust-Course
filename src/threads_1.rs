use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn learning_threads() {
    println!("Start to learn threads 1");

    // 无畏并发
    // 并发
    // Concurrent: 程序的不同部分之间独立的运行
    // Parallel:   程序的不同部分同时运行

    // Rust 无畏并发: 允许你编写没有细微 Bug 的代码, 并在不引入新 bug 的情况下易于重构
    // 注意: "并发" 指的是 Concurrent 和 Parallel

    // 使用线程同时运行代码

    // 进程与线程
    // 在大部分 OS 里, 代码运行在进程 ( process ) 中, OS 同时管理多个进程
    // 在你的程序里, 各部分独立可以同时运行, 运行这些独立部分的就是线程 ( thread )
    // 多线程运行:
    //  - 提升性能表现
    //  - 增加复杂性: 无法保障各线程的执行顺序

    // 多线程可导致的问题
    // 竞争状态, 线程以不一致的顺序访问数据或资源
    // 死锁, 两个线程彼此等待对方使用完所持有的资源, 线程无法继续
    // 只在某些情况下的 Bug, 很难可靠地复制现象和修复

    // 实现线程的方式
    // 通过 OS 的 API 来创建线程:  1:1 模型
    //  - 需要较小的运行时
    // 语言自己实现的线程 ( 绿色线程 ):  M:N 模型
    //  - 需要更大的运行时

    // Rust: 需要权衡运行时的支持
    // Rust 标准库仅提供 1:1 模型的线程

    // 当主线程结束的时候, 无论分线程是否完成执行, 程序都会结束

    // 通过 spawn 创建新线程
    // 通过 thread:spawn 函数创建新线程:
    //  - 参数: 一个闭包 ( 在新线程里运行的代码 )
    //  - thread::sleep 会导致当前线程暂停执行

    // 通过 join handle 来等待所有线程的完成
    //  - thread::spawn 函数的返回值类型是 JoinHandle
    //  - JoinHandle 持有值的所有权
    //     - 调用其 join  方法, 可以等待对应的其他线程的完成
    // join 方法: 调用 handle 的 join 方法会阻止当前线程的执行, 直到 handle 所表示的这些线程终结

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // join 会阻塞主线程的执行, 直到分线程执行完成后
    // 这个函数写在哪，这个函数对应的 thread 就会在哪里执行完毕
    // 比如, 目前 join 函数在 main 线程代码之前, 所以在分线程会执行完成之后再执行下方的代码
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 使用 move 闭包
    // move 闭包通常和 thread::spawn 函数一起使用, 它允许你使用其他线程的数据
    // 创建线程时, 把值的所有权从一个线程转移到另一个线程

    let v = vec![1, 2, 3];

    // closure may outlive the current function, but it borrows `v`, which is owned by the current function
    // 下面的代码会报错, 因为闭包的生命周期有可能比 v 要来的长, 比如我们可以在 80 行执行 drop(v) 代码来提前释放 v
    // 解决方法是在闭包前加 move 关键字
    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });

    handle.join().unwrap();
}