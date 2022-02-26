use std::rc::Rc;
use std::cell::RefCell;

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
    // a rc count after b creation = 1, todo: 为什么此处是 1
    println!("a rc count after b creation = {}", Rc::strong_count(&b));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
}