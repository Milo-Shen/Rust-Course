use std::rc::Rc;
use std::{borrow::BorrowMut, cell::RefCell};

pub trait Messenger {
    fn send(&self, msg: &str);
}

// todo: 了解 'a + Messenger 用法的含义
pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

// 为什么 Where 子句此处不用 where T: 'a + Messenger
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        // todo: 为什么这里要使用 as f64 转换成 64 位浮点再执行除法
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: you are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quote");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Urgent warning: You've used up over 75% of your quote");
        }
    }
}

#[derive(Debug)]
enum List {
    Cons(RefCell<i32>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn learning_ref_cell() {
    println!("Start to learn RefCell<T>");

    // RefCell<T> 和内部可变性
    // 内部可变性 ( interior mutability )
    // 内部可变性是 Rust 的设计模式之一
    // 它允许你在只持有不可变引用的前提下对数据进行修改
    //  - 数据结构中使用了 unsafe 代码来绕过 Rust 正常的可变性和借用规则

    // RefCell<T>
    // 与 Rc<T> 不同, RefCell<T> 类型代表了其持有数据的唯一所有权

    // 回忆一下借用规则
    // 在任何给定的时间, 你要么只能拥有一个可变引用, 要么只能拥有任意数量的不可变引用
    // 引用总是有效的

    // RefCell<T> 与 Box<T> 的区别
    // Box<T>                       RefCell<T>
    //  - 编译阶段强制代码遵守借用规则    - 只会在运行时检查借用规则
    //  - 否则出现编译错误              - 否则触发 panic

    // 借用规则在不同阶段进行检查的比较
    // 编译阶段                      运行时
    //  - 尽早暴露问题                 - 暴露问题延后, 甚至到生产环境
    //  - 没有任何运行时开销            - 因借用计数产生些许性能损失
    //  - 对大多数场景是最佳选择         - 实现某些特定的内存安全场景 ( 不可变环境中修改自身数据 )
    //  - 是 Rust 的默认行为

    // Rust 编译器是非常保守的, Rust 代码会拒绝掉所有不符合所有权规则的代码, 哪怕这些代码完全没有任何问题
    // 针对编译器无法理解的代码, 但是开发者可以保证代码的正确性, 那么 RefCell<T> 就有用武之地了

    // RefCell<T>
    // 与 Rc<T> 相似, 只能适用于单线程场景

    // 选择 Box<T>、Rc<T>、RefCell<T> 的依据
    //                      Box<T>            Rc<T>            RefCell<T>
    // 同一数据的所有者        一个              多个              一个
    // 可变性、借用检查        可变、不可变借用    不可变借用         可变、不可变借用
    //                      ( 编译时检查 )     ( 编译时检查 )     运行时检查
    //
    // 其中: 即便 RefCell<T> 本身不可变, 但仍能修改其中存储的值

    // 内部可变性: 可变的借用一个不可变的值
    // 借用规则有一个推论: 无法可变的借用一个不可变的值

    let x = 5;
    // 借用规则有一个推论: 无法可变的借用一个不可变的值, 所以下面代码错误
    // error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
    // let y = &mut x;

    // 将 Rc<T> 和 RcCell<T> 结合使用来实现一个拥有多重所有权的可变数据
    let a = Rc::new(Cons(RefCell::new(5), Rc::new(Nil)));
    let b = Cons(RefCell::new(6), Rc::clone(&a));
    let c = Cons(RefCell::new(9), Rc::clone(&a));

    // todo: 理解此处 * 的用法, 下面这句话的执行顺序是否是: *(value.borrow_mut()) += 10;
    match b {
        List::Cons(a, _) => {
            *a.borrow_mut() += 100;
        }
        _ => (),
    }
    println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // 其他可实现内部可变性的类型
    //  - Cell<T>: 通过复制来访问数据
    //  - Mutex<T>: 用于实现跨线程情形下的内部可变性模式
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // borrow_mut 可以获得内部值的可变引用
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        // borrow 可以获得内部值的不可变引用
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    // 使用 RefCell<T> 在运行时记录借用信息
    // 两个方法 ( 安全接口 )
    //  - borrow 方法: 返回智能指针 Ref<T>, 它实现了 Deref
    //  - borrow_mut 方法: 返回智能指针 RefMut<T>, 它实现了 Deref
    // RefCell<T> 会记录当前存在多少个活跃的 Ref<T> 和 RefMut<T> 智能指针:
    //  - 每次调用 borrow: 不可变借用计数 + 1
    //  - 任何一个 Ref<T> 的值离开作用域被释放时: 不可变借用计数 -1
    //  - 每次调用 borrow_mut: 可变借用计数 + 1
    //  - 任何一个 RefMut<T> 的值离开作用域被释放时: 可变借用计数 - 1

    // 以此技术来维护借用检查规则:
    //  - 任何一个给定时间里, 只允许拥有多个不可变你借用或一个可变借用
    // todo: 能否同时拥有多个不可变借用和一个可变借用

    // 将 Rc<T> 和 RcCell<T> 结合使用来实现一个拥有多重所有权的可变数据
}
