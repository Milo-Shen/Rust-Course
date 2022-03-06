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
    }

    let color = ColorPanel::ChangeColor(Color::RGB(0, 0, 0));
    match color {
        ColorPanel::ChangeColor(Color::RGB(r, g, b)) => {
            println!("change the color: r = {} ,g = {} ,b = {}", r, g, b);
        }
        ColorPanel::ChangeColor(Color::HSV(h, s, v)) => {
            println!("change the color: h = {} ,s = {} ,v = {}", h, s, v);
        }
    }
}