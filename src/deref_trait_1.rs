use std::ops::Deref;

pub fn learning_deref_trait() {
    println!("Start to learn deref trait 1");

    // Deref 是解引用的意思
    // 实现 Deref Trait 使我们可以自定义解引用运算符 * 的行为
    // 通过实现 Deref, 智能指针可以像常规引用一样处理

    // 解引用运算符
    // 常规的引用也是一种指针

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // y 是一个指针, * 是解引用运算符, 用于获取指针指向的内存地址里的值
    assert_eq!(5, *y);

    // 把 Box<T> 当做引用
    // Box <T> 可以代替上例中的引用
    let z = Box::new(x);
    // 如果不加 * 的话: ^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
    assert_eq!(5, *z);

    // 定义自己的智能指针
    // Box<T> 被定义成拥有一个元素的 tuple struct
    // 例子: MyBox<T>

    // 定义一个 tuple struct
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    let a = MyBox::new(x);
    // error[E0614]: type `MyBox<{integer}>` cannot be de-referenced
    // 因为 MyBox 没有实现 Deref Trait, 所以下面的解引用代码报错
    // assert_eq!(5, *a);

    // 实现 Deref Trait
    // 标准库中的 Deref trait 要求我们实现一个 deref 方法:
    //  - 该方法借用 self
    //  - 返回一个指向内部数据的引用 ( todo: 不会报错么 ? )

    impl<T> Deref for MyBox<T> {
        // 使用 Type 定义了一个关联类型
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    assert_eq!(5, *a);

    // Rust  会自动把实现了 Deref Trait 的自定义类型 MyBox 展开成如下的模式:
    assert_eq!(5, *(y.deref()));
}
