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
    let mut jason = User { age: 65, ..jack };
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
        // 能不能不加 & 这个符号 ?
        // 不能: error[E0382]: use of moved value: `rect`
        // 不加 & 的话，执行 area 方法后，当前实例所有权会转移到 area 这个方法，后续的代码中将无法继续访问
        // 通常来说，将第一个参数标记为 self 并在调用过程中取得实例所有权的方法并不常见。这种技术有可能会被用于那些
        // 需要将 self 转换为其他类型, 且在转换之后想要阻止调用者访问原始实例的场景
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

    // 运算符 -> 去哪了的实例
    // 在 C 和 C++ 中调用方法有 2 个不同的运算符:
    //  1. 直接用于对象本身的 .
    //  2. 用于对象指针的 ->
    // 之所以有这样的区别，是因为我们在调用指针的方法时首先需要对该指针进行解引用。换句话说, 假如 object 是一个指针，那么 object -> something() 的写法实际上等价于 (*object).something()
    // Rust 没有提供这样的运算符
    // 在 rust 中, 设计了一种自动引用和解引用的功能。方法调用是 Rust 中少数几个拥有这种行为的地方之一。
    // 他的工作模式如下: 当你使用 object.something() 调用方法时, rust 会自动为调用者 object 添加 &, &mut 或 * 使其能够符合签名的方法。换句话说，下面 2 种调用是等价的:
    //  1: p1.distance(&p2);
    //  2. (&p1).distance(&p2);
    // 第一种调用要简洁得多，则、何种自动解引用行为之所以能够行得通，是因为该方法有一个明确的作用对象: self 的类型。在给出调用者和方法名的前提下，Rust 可以准确地推导出:
    //  1. 方法是否是只读的 (&self)
    //  2. 是否需要修改数据 (&mut self)
    //  3. 是否会获取数据的所有权 (self)
    // 这种针对方法调用者的隐式借用在实践中可以让所有权系统更加友好且易于使用
}
