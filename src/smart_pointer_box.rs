fn take_ownership(x: Box<i32>) {
    println!("take ownership: {}", x);
}

pub fn learning_smart_pointer() {
    println!("Start to learn smart pointer - Box<T>");

    // 使用 Box<T> 来指向 Heap 上的数据
    // Box<T> 是最简单的智能指针:
    //  - 允许你在 heap 上存储数据 ( 而不是 stack )
    //  - stack 上是指向 heap 数据的指针
    //  - 没有性能开销
    //  - 没有其他额外功能
    //  - 实现了 Deref trait 和 Drop trait

    // Box<T> 的常用场景
    // 在编译时，某类型的大小无法确定。但使用该类型时，上下文需要知道它的确切大小
    // 当你有大量数据，想移交所有权，但需要确保在操作数据时数据不会被复制
    // 使用某个值时，你只关心它是否实现了特定的 trait，而不关心它的具体类型

    // 使用 Box<T> 在 heap 上面存储数据
    let b: Box<i32> = Box::new(5);
    take_ownership(b);
    // Box 拥有对数据的所有权，所以此处当 b 丧失所有权后，无法被继续访问
    // borrow of moved value: `b`
    // println!("{}", b);

    // 使用 Box 赋能递归类型
    // 在编译时，Rust 需要知道一个类型所占的空间大小
    // 而递归类型的大小无法在编译时确定
    // 但 Box 类型的大小确定
    // 在递归类型中使用 Box 就可解决上述问题
    // 函数式语言中的 Cons List

    // 关于 Cons List ( Cons List 相当于一个链表 )
    //  - Cons List 是来自 Lisp 语言的一种数据结构
    // Cons List 里每个成员由 2 个元素组成
    //  - 当前项的值
    //  - 下一个元素
    // Cons List 里最后一个成员只包含一个 Nil 值，没有下一个元素
    // Nil 是一个终止标记，和 Null 不一样，Null 表示的是一个无效或是缺失的值

    // Cons List 并不是 Rust 的常用集合
    // 通常情况下, Vec<T> 是更好的选择

    // 例子: Rust 如何确定为枚举类型分类的空间大小
    // Rust 会遍历枚举中的每个变体，从而寻找需要最大空间的那个变体
    enum Message {
        // 在 rust 眼中，Quit 这个变体不需要占用任何空间
        Quit,
        // Move 占用的空间为 2 个 i32 类型所占用的空间
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // 例子: 创建一个 Cons List
    // 下面这个 enum 的申明会报错: error[E0072]: recursive type `List` has infinite size
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }

    // 使用 Box 来获得确定大小的递归类型
    // Box<T> 是一个指针, Rust 知道它需要多少空间, 因为:
    //  - 指针的大小不会基于它指向的数据的大小变化而变化

    // Box<T>:
    //  - 只提供了 "间接" 存储和 heap 内存分配的功能
    //  - 没有其他额外功能
    //  - 没有性能开销
    //  - 适用于需要 "间接" 存储的场景, 例如: Cons List
    //  - 实现了 Deref trait 和 Drop Trait 这两个 trait
    //  - Deref: 允许我们将 Box 的值当做引用来处理
    //  - Drop: 当 Box 值离开作用于时, 它所指向的 heap 内存上面的数据以及指针数据都会被自动清理掉
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};
    let list: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    let box_str = Box::new(String::from("hello"));
    // 此时，Box 内部的 String 的所有权就转移给了 unbox_str
    let unbox_str = *box_str;
    // borrow of moved value: `box_str`
    // println!("{}", box_str);
}
