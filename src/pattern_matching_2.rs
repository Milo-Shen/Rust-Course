pub fn learning_pattern_matching() {
    println!("start to learn pattern matching 2");

    // 课辩驳性: 模式是否会无法匹配

    // 模式的两种形式
    // 能匹配任何可能传递的值的模式: 无可辩驳的
    //  - 例如: let x = 5;
    // 对于某些可能的值, 无法进行匹配的模式: 课辩驳的
    //  - 例如: if let Some(x) = a_value
    // 函数参数、let 语句、for 循环只接受无可辩驳的模式
    // if let 和 while let 接受可辩驳和无可辩驳的模式
    let a: Option<i32> = Some(5);
    // error[E0005]: refutable pattern in local binding: `None` not covered
    // pattern `None` not covered
    // let 语句只接受可辩驳的模式, 所以下面的代码会报错
    // let Some(x) = a;
    // 修正后的代码:
    if let Some(x) = a {
        println!("The value of x is: {}", x);
    }

    // 下面的模式匹配没有任何意义, 因为 x = 5 是一个无可辩驳的模式, 永远不会失败
    // 所以没有必要使用可辩驳的模式匹配
    if let x = 50 {
        println!("The value of x is: {}", x);
    }

    // match 表达式
    // match 表达式除了最后一个分支, 其他的分支都必须是可辩驳的也就是可失败的,
    // 最后一个分支是不可辩驳的,不可失败的, 因为它要匹配所有剩余的情况
}
