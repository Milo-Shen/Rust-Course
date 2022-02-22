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
    // 借用规则有一个推论: 可变的借用一个不可变的值

    let x = 5;
    // 借用规则有一个推论: 可变的借用一个不可变的值, 所以下面代码错误
    // error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
    // let y = &mut x;

    // 下面我们来看一个例子
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
        where T: Messenger {
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
        }
    }
}
