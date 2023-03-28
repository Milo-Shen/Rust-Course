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

    // 结构体更新语法
    let mut marry = User {
        name: String::from("Mary"),
        ..jack
    };
    println!("jack name: {}, marry name: {}", jack.name, marry.name);
    marry.age = 99;

    // 结构体更新语法，并不会共享值，值是复制过去的
    println!("jack age: {}, marry age: {}", jack.age, marry.age);

    // update the value of struct User for jack variable
    jack.name = String::from("Jack Upgrade");
    println!("name: {}, {}", jack.name, marry.name);

    // jack.name 的所有权已经被转移到了 jason 中
    let mut jason = User {
        age: 65,
        ..jack
    };
    println!("Jason name: {}", jason.name);
    // error[E0382]: borrow of moved value: `jack.name`
    // 下面这句话会报错，因为 jack.name 的所有权已经被转移到了 jason 中
    // println!("{}", jack.name);

    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(f32, f32, f64);
    let black: Color = Color(0, 0, 0);
    let yellow: Color = Color(1, 1, 1);
    let point: Point = Point(1.0, 1.0, 1.0);

    // an implementation of `PartialEq<_>` might be missing for `Color`an implementation of `PartialEq<_>` might be missing for `Color`
    // let is_equal = black == yellow;
    // println!("black.0: {}, point.0: {}, isEqual: {}", black.0, point.0, is_equal);

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

    let mut rect = Rectangle {
        width: 30,
        height: 40,
    };

    println!("The area of rect is: {}", area(&rect));
    println!("The rect is: {:#?}", rect);


    // Method of struct
    impl Rectangle {
        // todo: 能不能不加 & 这个符号 ?
        // 不能: error[E0382]: use of moved value: `rect`
        // 不加 & 的话，执行 area 方法后，当前实例所有权会转移到 area 这个方法，后续的代码中将无法继续访问
        fn area(&self) -> u32 {
            return self.width * self.height;
        }

        fn copy(&self) -> Rectangle {
            return Rectangle {
                width: self.width,
                height: self.height,
            };
        }

        // 关联函数
        fn print(x: &str) {
            println!("static function print: {}", x);
        }
    }

    let area = rect.area();
    println!("The area is: {}", area);
    Rectangle::print("My Print");
    let mut another_react = &mut rect;
    let mut another_react = another_react.copy();
    another_react.width = 10000;
    println!("origin value of rect is: {}", rect.width);
}