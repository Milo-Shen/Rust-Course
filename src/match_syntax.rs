pub fn learning_match() {
    println!("Start to learn match");

    // match 关键字
    // 允许一个值与一系列模式进行匹配, 模式
    #[derive(Debug)]
    enum China {
        Shanghai,
        Beijing,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
        RMB(China),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Coin::Penny has been called");
                return 1;
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::RMB(state) => {
                println!("State quarter from : {:?}", state);
                return 25;
            }
            _ => 15
        }
    }

    let my_coin = Coin::RMB(China::Beijing);
    let result = value_in_cents(my_coin);
    println!("The value of Coin::Penny is: {}", result);

    // Rust 的 match 匹配，必须穷举所有的可能性
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1)
        }
    }

    let v: u8 = 10u8;
    let matched_value = match v {
        0 => 1,
        1 => 2,
        2 => 3,
        _ => 100,
    };
    println!("matched value: {}", matched_value);


    // 模式与模式匹配 if let
    // if let 只关心一种匹配，而忽略其他匹配的情况，可以看做是 match 的语法糖
    // if let 可以匹配具体的值, 那么是否要实现 Equal 这个 Trait 才行 ?
    let v = Some(0);
    match v {
        Some(3) => println!("match three"),
        _ => println!("match others"),
    }

    // if let 只关心一种匹配，而忽略其他匹配的情况，可以看做是 match 的语法糖
    if let Some(3) = v {
        println!("if let three")
    } else {
        println!("if let others")
    }

    // if let 可以从枚举中把值取出来
    if let Some(x) = v {
        println!("if let val: {}", x);
    }

    let v = China::Beijing;
    // 可以通过这种方式，对枚举值进行判断
    // 虽然作为 match 的语法糖，但是用处了条件语句的感觉
    if let China::Shanghai = v {
        println!("if let China::Beijing");
    } else {
        println!("if let China::else Shanghai");
    }

    // 试试看非内置类型的情况
    enum HighContrast {
        White,
        Black,
    }

    let hcb = HighContrast::White;
    // 需要特别注意的是: HighContrast::Black = hcb 不能倒过来写
    // 判断的类型放前面，具体的值放后面
    if let HighContrast::Black = hcb {
        println!("This is hcb");
    } else {
        println!("This is else hcw");
    }

    // 模拟三目运算符
    let num = 1;
    if num == 1 { println!("1") } else { print!("2"); }
}