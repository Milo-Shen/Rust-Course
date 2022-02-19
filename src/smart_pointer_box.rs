pub fn learning_smart_pointer() {
    println!("Start to learn smart pointer - Box<T>");

    // 使用 Box<T> 来指向 Heap 上的数据
    // Box<T> 是最简单的智能指针:
    //  - 允许你在 heap 上存储数据 ( 而不是 stack )
    //  - stack 上是指向 heap 数据的指针
    //  - 没有性能开销
    //  - 没有其他额外功能
    //  - 实现了 Deref trait 和 Drop trait
}