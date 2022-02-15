pub fn learning_closure() {
    println!("Start to learn closure 2");

    // 闭包可以捕获他们所在的环境
    let x: i32 = 1;

    // 函数无法捕获环境中的变量
    // error[E0434]: can't capture dynamic environment in a fn item
    // fn get_env_by_fn(m: i32) -> bool {
    //     return m == x;
    // }

    // 闭包可以捕获他们所在的环境
    //  - 闭包可以访问定义它的作用域内的变量，而普通的函数则不能
    //  - 使用闭包会产生内存开销

    // 闭包从所在环境捕获值的方式
    // 与函数获得参数的三种方式:
    //   1. 取得所有权: FnOnce
    //   2. 可变借用:   FnMut
    //   3. 不可变借用: Fn
    let equal_to_x = |z: i32| z == x;
    println!("The result of equal_to_x is: {}", equal_to_x(1));

    // 创建闭包时，通过闭包对环境值的使用，Rust 推断出具体使用哪个 trait
    //  - 所有的闭包都实现了 FnOnce
    //  - 没有移动被捕获变量的闭包实现了 FnMut ( 实现包含了前者 )
    //  - 无需可变访问被捕获变量的闭包实现了 Fn  ( 实现包含了前 2 者 )
    let mut string_1 = String::from("FnOnce");
    let equal_to_str = |z: String| z == string_1;
    // cannot borrow `string_1` as mutable because it is also borrowed as immutable
    // string_1.push_str("2");
    println!("The result of equal_to_str is: {}", equal_to_str(String::from("FnOnce")));
    // error[E0506]: cannot assign to `string_1` because it is borrowed
    // string_1 = String::from("Fn");
    println!("The result of equal_to_str is: {}", equal_to_str(String::from("FnOnce")));
    println!("The value: {} is still available", string_1);

    // move 关键字
    // 在参数列表前使用 move 关键字，可以强制闭包取得它所使用的环境的所有权
    //  - 当将闭包传递给新线程以移动数据使其归新线程所有时，此技术最为有用
    let x = vec![1, 2, 3];
    let mut string_1 = String::from("FnOnce");
    let equal_to_str = move |z: String| z == string_1;
    // borrow of moved value: `string_1`
    // println!("The value: {} is still available", string_1);
}