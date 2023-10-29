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
        _ => println!("Default case, x = {:?}", x),
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

    // 解构赋值时候, 新增 mut 控制
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    println!("a = {:?}, b = {:?}", a, b);

    // 1.59 之后结构赋值的新用法
    // struct Struct {
    //     e: i32,
    // }
    //
    // let (a, b, c, d, e);
    // (a, b) = (1, 2);
    // // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有是一个变量名而是使用了 _
    // [c, .., d, _] = [1, 2, 3, 4, 5];
    // Struct { e, .. } = Struct { e: 5 };

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
        _ => setting_value = new_setting_value,
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

    // 此处使用 _ 进行解构时不会发生绑定这类操作, 不会移动所有钱
    // 所有权不会发生移动
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // 若是使用非 _变量, 则会发生绑定操作
    // 所有权会发生移动，下面 s 变量的所有权被转移到了 _s 中
    if let Some(_s) = s {
        println!("found a string");
    }
    // println!("{:?}", s);

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

    // 使用 match 守卫来提供额外的条件
    // match 守卫就是 match arm 模式后额外的 if 条件, 想要匹配该条件也必须满足
    // match 守卫适用于比单独的模式更复杂的场景
    let num = Some(4);
    // 下面的代码中, Some(x) 就是模式, x < 5 就是 match 守卫
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(10);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 配合多重模式来使用 match 守卫
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ 绑定
    // @ 符号让我们可以创建一个变量, 该变量可以在测试某个值是否与模式匹配的同时保存该值
    enum HelloMessage {
        Hello { id: i32 },
    }
    let msg = HelloMessage::Hello { id: 5 };

    match msg {
        HelloMessage::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable);
        }
        HelloMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        }
        HelloMessage::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
}
