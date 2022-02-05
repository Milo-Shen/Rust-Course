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
    // error[E0369]: binary operation `==` cannot be applied to type `IpAddressKind`
    // let is_equal = home == home_another;

    // option 枚举 (位于预导入模块重 prelude)
    // Rust 中没有 NULL
    let some_number = Some(5);
    let some_string = Some("A String");
    let absent_number: Option<i32> = None;
}