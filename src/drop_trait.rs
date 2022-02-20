pub fn learning_drop_trait() {
    println!("Start to learn drop trait");

    // Drop Trait
    // 实现 Drop Trait, 可以让我们自定义当值将要离开作用于时发生的动作 ( 有点类似于析构函数 )
    //  - 例如: 文件、网络资源的释放等
    //  - 任何类型都可以实现 Drop trait

    // Drop trait 只要求你实现 drop 方法
    //  - 参数: 对 self 的可变引用

    // Drop trait 在与导入模块里 ( prelude )

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        // 此处的 self 一定是 &mut 类型的，否则会报错
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data: {}", self.data);
        }
    }

    // 输出:
    // Dropping CustomSmartPointer with data: other stuff
    // Dropping CustomSmartPointer with data: my stuff
    // 释放资源的顺序和申明资源的顺序是相反的
    let a = CustomSmartPointer { data: String::from("my stuff") };
    let b = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointer created.");

    // 使用 std::mem::drop 来提前 drop 值
    // 很难直接禁用自动的 drop 功能，也没必要
    //  - Drop trait 的目的就是进行自动的释放处理逻辑
    // Rust 不允许手动调用 Drop trait 的 drop 方法
    // 但可以调用标准库的 std::mem::drop 函数，来提前 drop 值

    // error[E0040]: explicit use of destructor method
    // a.drop();

    // 使用标准库的 std::mem::drop 函数，来提前 drop 值
    let c = CustomSmartPointer { data: String::from("my stuff drop") };
    let d = CustomSmartPointer { data: String::from("other stuff drop") };

    // 输出:
    // Dropping CustomSmartPointer with data: my stuff drop
    // Dropping CustomSmartPointer with data: other stuff drop
    // 因为手动调用了 drop 方法, 所以 my stuff drop 会被先释放
    // Rust 里所有权系统会保证 drop 方法不存在重复释放，也就是 double free 这种错误的发生
    drop(c);
}
