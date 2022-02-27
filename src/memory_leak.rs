use std::rc::{Rc, Weak};
use std::cell::{Ref, RefCell};

// 如此我们可以灵活修改 Cons 来让他指向下一个 list 值
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

pub fn learning_memory_leak() {
    println!("Start to learn memory leak");

    // Rust 可能发生内存泄漏
    // Rust 的内存安全机制可以保证很难发生内存泄漏, 但不是不可能
    // 例如使用 Rc<T> 和 RefCell<T> 就可能创造循环引用, 从而发生内存泄漏
    //  - 每个项的引用数量不会变成 0, 值也不会被处理掉

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    // a rc count after b creation = 2
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // a-> 5 -> b -> 10 -> a 循环引用，所以下面的这行代码会堆栈溢出
    // println!("a next item = {:?}", a.tail());

    // 防止内存泄漏的解决办法
    // 依靠开发者来保证, 不能依靠 rust
    // 重新组织数据结构: 一些引用来表达所有权, 一些引用不表达所有权
    //  - 循环引用中的一部分具有所有权关系, 另一部分不涉及所有权关系
    //  - 只有所有权关系才影响值的清理

    // 防止循环引用 - 把 Rc<T> 换成 Weak<T>
    // 把 Rc::clone 为 Rc<T> 实例的 strong_count + 1, Rc<T> 的实例只有在 strong_count 为 0 时才会被清理
    // Rc<T> 实例通过调用 Rc::downgrade 方法可以创建 Weak Reference ( 弱引用 )
    //  - 返回类型是 Weak<T> ( 智能指针 )
    //  - 调用 Rc::downgrade 会为 weak_count + 1
    // Rc<T> 使用 weak_count 来追踪存在多少 Weak<T>
    // weak_count 不为 0 并不影响 Rc<T> 实例的清理

    // Strong vs Weak
    // Strong Reference ( 强引用 ) 是如何分享 Rc<T> 实例的所有权
    // Weak Reference ( 弱引用 ) 并不表达上述意思
    // 使用 Weak Reference 并不会创建循环引用:
    //  - 当 Strong Reference 数量为 0 的时候, Weak Reference 会自动断开
    // 在使用 Weak<T> 前, 需保证它指向的值仍然存在:
    //  - 在 Weak<T> 实例上调用 upgrade 方法, 返回 Option<Rc<T>>
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    // 目前 leaf 里的 node 有两个所有者, 一个是 leaf, 另一个是 branch 的 children
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent  = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // 因为使用了 Weak, 所以下面这段代码没有造成循环引用
    println!("leaf parent  = {:#?}", leaf.parent.borrow().upgrade());
}