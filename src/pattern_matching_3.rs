pub fn learning_pattern_matching() {
    println!("start to learn pattern matching 3");

    // 模式语法
    // 匹配字面值 - 模式可以直接匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }

    // 匹配命名变量
    // 命名的变量是可匹配任何值的无可辩驳模式
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {}", y),
        _ => println!("Default case, x = {:?}", x)
    }
    println!("At the end: x= {:?},y = {}", x, y);
}