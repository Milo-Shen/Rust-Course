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

    // 多重模式
    // 在 match 表达式中, 使用 | 语法 ( 就是或的意思 ), 可以匹配多种模式
    let x = 1;
    match x {
        1 | 2 => println!("one"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // 使用 ..= 来匹配某个范围的值
    let x = 5;
    match x {
        1..=5 => println!("1 ~ 5"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 结构以分解值
    // 可以使用模式来解构 struct、enum、tuple, 从而引用这些类型值的不同部分
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    // 输出: a = 0, b = 7
    println!("a = {}, b = {}", a, b);
    let Point { x, y } = p;
    println!("x = {}, y = {}", x, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis ({},{})", x, y),
    }

    // 解构枚举 enum
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The quit variant has no data to destructure");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text Message {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("change the color: r = {} ,g = {} ,b = {}", r, g, b);
        }
    }

    // 结构嵌套的 struct 和 enum
    enum Color {
        RGB(i32, i32, i32),
        HSV(i32, i32, i32),
    }

    enum ColorPanel {
        ChangeColor(Color),
        Position(Point),
    }

    let color = ColorPanel::ChangeColor(Color::RGB(0, 0, 0));
    match color {
        ColorPanel::ChangeColor(Color::RGB(r, g, b)) => {
            println!("change the color: r = {} ,g = {} ,b = {}", r, g, b);
        }
        ColorPanel::ChangeColor(Color::HSV(h, s, v)) => {
            println!("change the color: h = {} ,s = {} ,v = {}", h, s, v);
        }
        _ => println!("other cases"),
    }

    let pos = ColorPanel::Position(Point { x: 1, y: 2 });
    match pos {
        ColorPanel::Position(Point { x, y }) => println!("The current pos is x = {} ,y = {}", x, y),
        _ => println!("other cases"),
    }

    // 解构 struct 和 tuple
    let ((feat, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feat = {}, inches = {}, x = {}, y = {}", feat, inches, x, y);

    // 在模式中忽略值
    //  - 使用 _ 来忽略整个值
    //  - 使用 _ 配合其他模式可以用来忽略部分值
    //  - 忽略使用 _ 开头的名称
    //  使用 .. 来忽略值的剩余部分

    // 使用 _ 来忽略整个值
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter = {}", y);
    }
    foo(1, 2);

    // 使用嵌套的 _ 来忽略值的一部分
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {},{},{}", first, third, fifth);
        }
    }

    // 通过使用 _ 开头命名来忽略未使用的变量
    let _x = 5;
    println!("_x = {}", _x);

    let s = Some(String::from("Hello !"));
    // 此处使用 _ 进行解构时不会发生绑定这类操作, 若是使用非 _ 变量, 则会发生绑定操作
    // 所有权将发生移动
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // 使用 .. 来忽略值的剩余部分
    let origin = Point { x: 1, y: 2 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("first = {}, last = {}", first, last);
        }
    }
}