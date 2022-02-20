use std::rc::Rc;

pub fn learning_rc() {
    println!("Start to learn Rc<T>");

    // Rc<T> 引用计数智能指针
    // 有时，一个值会有多个所有者
    // 为了支持多重所有权: Rc<T>
    //  - reference counting ( 引用计数 )
    //  - 追踪所有到值的引用
    //  - 0 个引用: 该值可以被清理掉

    // Rc<T> 的使用场景
    // 需要在 heap 上分配数据，这些数据被程序的多个部分读取（ 只读 ）, 但在编译时无法确定哪个部分最后使用完这些场景
    // Rc<T> 只能用于单线程场景
    //  - Rc<T> 不在预导入模块 ( prelude )
    //  - Rc::clone(&a) 函数: 增加引用计数
    //  - Rc::strong_count(&a): 获得引用计数
    //  - Rc::weak_count 函数

    // 例子: 两个 List 共享另一个 List 的所有权
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list: Rc<List> = Rc::new(Cons(
        8, Rc::new(Cons(
            9, Rc::new(Cons(
                10, Rc::new(Nil)))))));
    println!("strong reference count after creating list = {}", Rc::strong_count(&list));

    // Rc::clone 不会执行数据的深度拷贝操作, 其只会增加引用计数, 而类似 list.clone() 方法会执行深度拷贝
    // Rc::clone method: This creates another pointer to the same allocation, increasing the strong reference count.
    let a = Cons(1, Rc::clone(&list));
    println!("strong reference count after creating a = {}", Rc::strong_count(&list));

    let b = Cons(2, Rc::clone(&list));
    println!("strong reference count after creating b = {}", Rc::strong_count(&list));

    {
        let c = Cons(3, Rc::clone(&list));
        println!("strong reference count after creating c = {}", Rc::strong_count(&list));
    }

    // 当上面的变量 c 离开作用域的时候, 其上的计数引用会自动减少 1, 因为 Rc<T> 也实现了 Drop Trait
    println!("count after c goes out of scope = {}", Rc::strong_count(&list));
}
