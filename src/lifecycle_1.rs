pub fn learning_lifecycle() {
    println!("Start to learn life cycle - 1");

    // 生命周期是对于引用而言的，Rust 的每个引用都有自己的生命周期
    // 生命周期: 引用保持有效的作用域
    // 大多数的情况：生命周期是隐式的、可被推断的
    // 当引用的生命周期可以以不同的方式相互关联时：手动标注生命周期
    // 生命周期存在的目的是避免悬垂引用

    let r: i32;
    {
        let x = 5;
        // r = &x; borrowed value does not live long enough
    }
    // 这里出错的原因是 r 的生命周期比 x 长，所以当此处访问 r 时，x 的生命周期早已结束
    // println!("r: {}", r);

    // Rust 的编译器使用借用检查器：来比较作用域以便判断所有的借用是否合法

    // 函数中的泛型生命周期
    let string_1 = String::from("Hello World");
    let string_2 = "Jack";

    // 生命周期的标注语法为 <'a> 配合使用 &'a，其中 a 可以替换成其他任意字符
    // 生命周期只和函数签名有关，和内部实现无关
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        return if x.len() > y.len() { x } else { y };
    }

    // 这里的 string_1.as_str() 也可以写成 &string_1，这样便用了解引用强制转换
    let result = longest(string_1.as_str(), string_2);
    println!("The longest value is: {}", result);
}
