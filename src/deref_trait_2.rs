use std::ops::Deref;

pub fn learning_deref_trait() {
    println!("Start to learn deref trait 2");

    // 函数和方法的隐式解引用转化 ( Deref Coercion )
    // 隐士解引用转化 ( Deref Coercion ) 是为函数和方法提供的一种便捷特性
    // 假设 T 实现了 Deref trait:
    //  - Deref Coercion 可以把 T 的引用转化为 T 经过 Deref 操作后生成的引用
    // 当把某个类型的引用传递给函数或方法时，但它的类型与定义的参数类型不匹配:
    //  - Deref Coercion 就会自动发生
    //  - 编译器对 deref 进行一系列调用, 把把它转化为所需的类型参数
    //  - 转化过程在编译时完成, 没有额外性能开销

    // 定义一个 tuple struct
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        // 使用 Type 定义了一个关联类型
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    fn hello(name: &str) {
        println!("Hello, {}", name)
    }

    let m = MyBox::new(String::from("Rust"));
    hello("Rust");

    // &m 的类型是 &MyBox<String>
    // deref -> &String
    // deref -> &str
    hello(&m);

    // 如果没有 Deref Coercion 方法, 那么代码要写成如下形式
    hello(&(*m)[..]);

    // 解引用与可变性
    // 可使用 DerefMut trait 重载可变引用的 * 运算符
    // 在类型和 trait在下列三种情况发生时, rust 会执行 deref coercion:
    //  - 当 T: Deref<Target=U>, 允许 &T 转换为 &U
    //  - 当 T: DerefMut<Target=U>, 允许 &mut T 转换为 &mut U
    //  - 当 T: Deref<Target=U>, 允许 &mut T 转换为 &U
}
