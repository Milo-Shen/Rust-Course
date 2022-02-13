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
    //   2. 可变借用
    //   3. 不可变借用: Fn
    let equal_to_x = |z| z == x;
    println!("The result of equal_to_x is: {}", equal_to_x(1));
    println!("The result of equal_to_x is: {}", equal_to_x(2));
}