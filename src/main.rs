use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

// use 的简写形式
use std::io::{self, Write};

mod front_of_house;

// 引入包内所有的内容
// use std::collections::*;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// mod tree
mod variables;
mod functions;
mod control_flow;
mod ownership;
mod reference;
mod slices;
mod hashmap;

fn main() {
    // Variables and Mutability
    variables::learning_variables();

    // Functions
    functions::learning_functions();

    // Control Flow
    control_flow::learning_control_flow();

    // Ownership
    ownership::learning_ownership();

    // Reference
    reference::learning_reference();

    // Slices
    slices::learning_slice();

    // struct
    struct User {
        name: String,
        age: u8,
    }
    let mut jack: User = User {
        age: 1,
        name: String::from("jack"),
    };
    let marry = User {
        name: String::from("Mary"),
        ..jack
    };
    println!("name: {}, {}", jack.name, marry.name);

    // update the value of struct User for jack variable
    jack.name = String::from("Jack Upgrade");
    println!("name: {}, {}", jack.name, marry.name);

    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(f32, f32, f64);
    let black: Color = Color(0, 0, 0);
    let point: Point = Point(1.0, 1.0, 1.0);
    println!("{}, {}", black.0, point.0);

    // Unit-Like Struct
    struct UnitLikeStruct {}

    // Struct Reference - LifeCycle
    // Missing lifetime specifier [E0106]
    // struct LifeCycle {
    //     name: &str,
    // }


    // The exercise of struct

    // 让 Rectangle 派生于 Debug
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn area(rect: &Rectangle) -> u32 {
        return rect.width * rect.height;
    }

    let rect = Rectangle {
        width: 30,
        height: 40,
    };

    println!("The area of rect is: {}", area(&rect));
    println!("The rect is: {:#?}", rect);


    // Method of struct
    impl Rectangle {
        fn area(&self) -> u32 {
            return self.width * self.height;
        }

        // 关联函数
        fn print(x: &str) {
            println!("static function print: {}", x);
        }
    }

    let area = rect.area();
    println!("The area is: {}", area);
    Rectangle::print("My Print");


    // 枚举与模式匹配
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


    // option 枚举 (位于预导入模块重 prelude)
    // Rust 中没有 NULL
    let some_number = Some(5);
    let some_string = Some("A String");
    let absent_number: Option<i32> = None;


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

    let v: u8 = 0u8;
    let matched_value = match v {
        0 => 1,
        1 => 2,
        2 => 3,
        _ => 100,
    };
    println!("matched value: {}", matched_value);


    // 模式与模式匹配 if let
    // if let 只关心一种匹配，而忽略其他匹配的情况，可以看做是 match 的语法糖
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

    let v = China::Beijing;
    // todo: 也许可以通过这种方式，对枚举值进行判断
    // todo: 虽然作为 match 的语法糖，但是用处了条件语句的感觉
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
    // todo: 需要特别注意的是: HighContrast::Black = hcb 不能倒过来写
    if let HighContrast::Black = hcb {
        println!("This is hcb");
    } else {
        println!("This is else hcw");
    }

    let num = 1;
    if num == 1 { println!("1") } else { print!("2"); }

    // struct, enum，其他: 指定完整的路径 （ 指定到本身 ）
    let mut map = HashMap::new();
    map.insert(1, 2);

    // 使用 module 下的函数
    front_of_house::hosting::print_house();

    // hashmap 章节
    hashmap::learning_hashmap();
}
