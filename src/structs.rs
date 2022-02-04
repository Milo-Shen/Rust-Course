pub fn learning_struct() {
    println!("Start to learn slices");

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
}