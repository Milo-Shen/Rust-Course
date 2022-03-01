use std::sync::{Mutex, Arc};
use std::thread;

pub fn learning_threads() {
    println!("Start to learn threads 3");

    // 共享状态的并发 ( 使用共享来实现并发 )
    // Go 语言的名言: 不要用共享内存来通信, 要用通信来共享内存
    // Rust 支持通过共享状态来实现并发
    // Channel 类似单所有权: 一旦将值的所有权转移至 Channel, 就无法使用它了
    // 共享内存并发类似多所有权: 多个线程可以同时访问同一块内存

    // 使用 Mutex 来每次只允许一个线程来访问数据
    // Mutex 是 mutual exclusion ( 互斥锁 ) 的缩写
    // 在同一时刻, Mutex 只允许一个线程来访问某些数据
    // 想要访问数据:
    //  - 线程必须先获取互斥锁 ( lock )
    //  - lock 数据结构式 mutex 的一部分, 它能跟踪谁对数据拥有独占访问权
    // mutex 通常被描述为: 通过锁定系统来保护它所持有的数据

    // Mutex 的两条规则
    // 在使用数据之前, 必须尝试获取锁 ( lock )
    // 使用完 mutex 所保护的数据, 必须对数据进行解锁, 以便其他线程可以获取锁

    // Mutex<T> 的 API
    // 通过 Mutex::new(数据) 来创建 Mutex<T>
    //  - Mutex<T> 是一个智能指针
    // 访问数据前, 通过 lock 方法获取锁
    //  - 会阻塞当前线程
    //  - lock 可能会失败
    //  - 返回的是 MutexGuard ( 智能指针, 实现了 Deref 和 Drop )
    let m = Mutex::new(5);
    {
        // lock 返回的是一个 MutexGuard<i32>, 它实现了 Deref, 所以它可以指向其内部的数据
        // 从而我们可以获取内部数据的引用, 因为该引用是可变的, 所以我们可以修改数据的值
        let mut num = m.lock().unwrap();
        *num = 6;
        // MutexGuard<i32> 这个智能指针也实现了 Drop Trait
        // 所以当代码走出 39 行作用域时, Mutex 会实现自动的解锁
    }
    println!("m = {:?}", m);

    // 多线程共享 Mutex<T> - 多线程的多重所有权
    // 使用 Arc<T> 来进行原子引用计数
    // Arc<T> 和 Rc<T> 类似, 它可以用于并发场景
    //  - A: atomic, 原子的
    // 为什么所有的基础类型都不是原子的, 为什么标准库类型不默认使用 Arc<T> ?
    //  - 需要性能作为代价
    // Arc<T> 和 Rc<T> 的 API 是相同的
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The value of count is = {}", *counter.lock().unwrap());

    // RefCell<T>/Rc<T>    vs    Mutex<T>/Arc<T>
    // Mutex<T> 提供了内部可变性, 和 Cell 家族一样
    // 我们使用 RefCell<T> 来改变 Rc<T> 里面的内容
    // 我们使用 Mutex<T> 来改变 Arc<T> 里面的内容
    // 注意: Mutex<T> 有死锁的风险
}