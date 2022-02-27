pub fn learning_threads() {
    println!("Start to learn threads 2");

    // 使用消息传递来跨线程传递数据

    // 消息传递
    // 一种很流行且能保证安全并发的技术就是: 消息传递
    //  - 线程 ( 或 actor ) 通过彼此发送消息 ( 数据 ) 来进行通信
    // Do 语言的名言: 不要用共享内存来通信, 要用通信来共享内存

    // Rust: Channel ( 标准库提供 )
    // Channel 包含: 发送端、接收端
}