pub fn learning_advanced_function() {
    println!("Start to learn advanced function and closure");

    // 高级函数和闭包
    // 可以将函数传递给其他函数
    // 函数在传递过程中会被强制转换成 fn 类型
    // fn 类型就是 "函数指针 ( function pointer )"
    fn add_one(x: u32) -> u32 { x + 1 }
    fn do_twice(f: fn(u32) -> u32, arg: u32) -> u32 { f(arg) + f(arg) }
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // 函数指针与闭包的不同
    // fn 是一个类型, 不是一个 trait
    //  - 可以直接指定 fn 为参数类型, 不用申明一个以 Fn trait 为约束的泛型参数
    // 函数指针实现了全部 3 种闭包 trait ( Fn, FnMut, FnOnce ):
    //  - 总是可以把函数指针用作参数传递给一个接收闭包的函数
    //  - 所以, 倾向于搭配闭包 trait 的泛型来编写函数: 可以同时接受闭包和普通函数
    // 某些情景, 只想接收 fn 而不接收闭包:
    //  - 与外部不支持闭包的代码交互: C 函数
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_numbers = vec![1, 2, 3];
}