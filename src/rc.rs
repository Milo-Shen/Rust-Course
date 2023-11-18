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

    let list: Rc<List> = Rc::new(Cons(8, Rc::new(Cons(9, Rc::new(Cons(10, Rc::new(Nil)))))));
    println!(
        "strong reference count after creating list = {}",
        Rc::strong_count(&list)
    );

    // Rc::clone 不会执行数据的深度拷贝操作, 其只会增加引用计数, 而类似 list.clone() 方法会执行深度拷贝
    // Rc::clone method: This creates another pointer to the same allocation, increasing the strong reference count.
    let a = Cons(1, Rc::clone(&list));
    println!(
        "strong reference count after creating a = {}",
        Rc::strong_count(&list)
    );

    let b = Cons(2, Rc::clone(&list));
    println!(
        "strong reference count after creating b = {}",
        Rc::strong_count(&list)
    );

    {
        let c = Cons(3, Rc::clone(&list));
        println!(
            "strong reference count after creating c = {}",
            Rc::strong_count(&list)
        );
    }

    // 当上面的变量 c 离开作用域的时候, 其上的计数引用会自动减少 1, 因为 Rc<T> 也实现了 Drop Trait
    println!(
        "count after c goes out of scope = {}",
        Rc::strong_count(&list)
    );

    // Rc::clone() vs 类型的 clone 方法
    // Rc::clone(): 增加引用, 不会执行数据的深度拷贝操作
    // 类型的 clone(): 很多会执行数据的深度拷贝操作

    // Rc<T>
    // Rc<T> 通过不可变引用, 使你在程序的不同部分之间共享只读数据
    //  - 若是 Rc<T> 允许持有多个可变引用的话, 那么它就会违反借用规则
    //  - 借用规则: 多个指向同一个区域的可变引用会导致数据的竞争，以及数据的不一致
    // 问题: 如何使得数据可变呢 ?

    // added in 2022-10-8
    let mut val = Rc::new(2);
    // todo: 如何体现 Rc<T> 仅允许编译时检查的不可变借用 ?
    let c = &mut val;
    // 此处的 clone 和 Rc::clone
    let some_val = Some(val.clone());
    // 外层的 Option clone 了，里面的 Rc 的强引用计数也会 +1
    let some_clone = some_val.clone();
    let another_clone = Rc::clone(&val);
    println!("option clone & Rc = {}", Rc::strong_count(&val));
}
