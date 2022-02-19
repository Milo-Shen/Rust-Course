pub fn learning_enums() {
    println!("Start to learn enums");

    enum IpAddKind {
        V4,
        V6,
    }

    let ipv4 = IpAddKind::V4;
    let ipv6 = IpAddKind::V6;
    let myv4 = IpAddKind::V4;
    // an implementation of `PartialEq<_>` might be missing for `IpAddKind`
    // let is_equal = ipv4 == myv4;
    // println!("is equal: {}", is_equal);

    fn route(ip_kind: IpAddKind) {}
    route(ipv4);
    route(ipv6);

    struct IpAddress {
        kind: IpAddKind,
        address: String,
    }

    enum IpAddressKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // todo: 下面这个值如何参与运算呢 ?
    let home = IpAddressKind::V4(127, 0, 0, 1);
    let home_another = IpAddressKind::V4(127, 0, 0, 2);
    let v6_address = IpAddressKind::V6(String::from("v6 address"));
    // error[E0369]: binary operation `==` cannot be applied to type `IpAddressKind`
    // let is_equal = home == home_another;

    // option 枚举 (位于预导入模块重 prelude)
    // Rust 中没有 NULL
    let some_number = Some(5);
    let some_string = Some("A String");
    let absent_number: Option<i32> = None;
    match v6_address {
        IpAddressKind::V4(a, b, c, d) => println!("{},{},{},{}", a, b, c, d),
        _ => println!("nothing"),
    }

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let my_message = Message::Move { x: 1, y: 2 };
    match my_message {
        Message::Move { x, y } => println!("{},{}", x, y),
        _ => println!("Nothing happen")
    }
    if let Message::Move { x, y } = my_message {
        println!("if let my_message: {},{}", x, y)
    }
}