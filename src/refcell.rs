pub fn learning_RefCell() {
    println!("Start to learn RefCell<T>");

    // RefCell<T> 和内部可变性
    // 内部可变性 ( interior mutability )
    // 内部可变性是 Rust 的设计模式之一
    // 它允许你在只持有不可变引用的前提下对数据进行修改
    //  - 数据结构中使用了 unsafe 代码来绕过 Rust 正常的可变性和借用规则
}
