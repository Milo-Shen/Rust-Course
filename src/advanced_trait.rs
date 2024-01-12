use std::cmp::Ordering;
use std::fmt;
use std::fmt::{write, Display, Formatter};
use std::ops::Add;
use std::process::Output;

pub fn learning_advanced_trait() {
    println!("Start to learn unsafe trait");

    struct Count {}

    // 在 trait 定义中使用关联类型来指定占位类型
    // 关联类型 ( associated type ) 是 Trait 中的类型占位符, 它可以用于 trait 的方法签名中:
    //  - 可以定义出包含某些类型的 trait, 而在实现前无需知道这些类型是什么
    pub trait MyIterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    impl MyIterator for Count {
        // 在具体实现 trait 的时候再指定类型
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    // 关联类型和泛型的区别
    //       泛型                                   关联类型
    // 每次实现 Trait 时标注类型                     无需标注类型 ( 但是要在里面指明具体的关联类型 )
    // 可以为一个类型多次实现某个 Trait                无法为单个类型多次实现某个 trait
    // ( 不同的泛型参数 )

    pub trait MyIterator2<T> {
        fn next(&mut self) -> Option<T>;
    }

    impl MyIterator2<String> for Count {
        fn next(&mut self) -> Option<String> {
            None
        }
    }

    impl MyIterator2<u32> for Count {
        fn next(&mut self) -> Option<u32> {
            None
        }
    }

    // 默认参数类型和运算符重载
    // 可以在使用泛型参数时为泛型指定一个默认的数据类型
    // 语法: <PlaceholderType=ConcreteType>
    // 这种计数常用于运算符重载 ( operator overloading )
    // Rust 不允许创建自己的运算符及重载任意的运算符
    // 但可以通过实现 std::ops 中列出的那些 trait 来重载一部分相应的运算符
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        // 下面的代码重载了 + 号运算符
        // 此处的 Point 也可以由 Self::Output 来代替
        fn add(self, rhs: Self) -> Point {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }

        // 此处的 Point 也可以由 Self::Output 来代替
        // fn add(self, rhs: Self::Output) -> Self::Output {
        //     Self::Output {
        //         x: self.x + rhs.x,
        //         y: self.y + rhs.y,
        //     }
        // }
    }

    impl Default for Point {
        fn default() -> Self {
            Self { x: 0, y: 0 }
        }
    }

    let point_1 = Point { x: 1, y: 2 };
    let point_2 = Point { x: 3, y: 4 };
    let my_pointer = Point { x: 1, y: 2 } + Point { x: 3, y: 4 };
    let is_point_equal = Point { x: 1, y: 2 } == Point { x: 1, y: 2 };
    println!("is Point equal: {}", is_point_equal);
    println!("The final point is: {:?}", my_pointer);

    // 如果，下面有自己的 PartialEq 实现则，无法再 derive PartialEq trait
    // 否则会引起: conflicting implementation for `Point1 问题
    // Eq 本身没有任何方法，它的作用在于表明被标记类型的每一个值都与自身相等 ( Eq trait 只能被应用在同时实现了 PartialEq trait 的类型上 )
    // 不是所有实现了 PartialEq 的类型都能实现 Eq，一个典型的例子是浮点数，浮点数中两个非数 Nan 值的实例是互不相等的
    #[derive(Debug, Eq, Copy)]
    struct Point1 {
        x: i32,
        y: i32,
    }

    impl Clone for Point1 {
        fn clone(&self) -> Self {
            println!("clone func of Point 1 has been called");
            Self { x: 0, y: 0 }
        }
    }

    impl PartialEq for Point1 {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    impl PartialOrd for Point1 {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let a = self.x * self.x + self.y + self.y;
            let b = other.x * other.x + other.y + other.y;

            // if a < b {
            //     return Some(Ordering::Less);
            // }
            // if a > b {
            //     return Some(Ordering::Greater);
            // }
            // return Some(Ordering::Equal);

            // 但是在给定值无法区分出次序时，返回 None
            return a.partial_cmp(&b);
        }
    }

    impl Ord for Point1 {
        fn cmp(&self, other: &Self) -> Ordering {
            let a = self.x * self.x + self.y + self.y;
            let b = other.x * other.x + other.y + other.y;
            return a.cmp(&b);
        }
    }

    let is_point_equal = Point1 { x: 1, y: 2 } == Point1 { x: 1, y: 3 };
    println!("is Point1 equal: {}", is_point_equal);
    let is_bigger = Point1 { x: 1, y: 2 } > Point1 { x: 1, y: 3 };
    println!("is bigger: {}", is_bigger);

    let mut new_point = Point1 { x: 10, y: 20 };
    let copied_point = new_point;
    new_point.x = 100;
    println!("new_point: {:?}", new_point);
    // 这里确实是生成了一个副本
    println!("copied_point: {:?}", copied_point);

    // 具体指明泛型参数类型的例子
    #[derive(Debug)]
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + (rhs.0 * 1000))
        }
    }

    let millimeters = Millimeters(1);
    let meter = Meters(2);
    let final_millimeters: Millimeters = millimeters + meter;
    println!("final_millimeters = {:?}", final_millimeters);

    // 默认泛型参数的主要应用场景
    // 扩展一个类型而不破坏现有代码
    // 允许在大部分用户都不需要的特定场景下进行自定义

    // 完全限定语法 ( Fully Qualified Syntax )
    // 如何调动同名方法
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("UP !");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;

    // 调用的是 Human 本身的 fly 方法
    // 执行结果为: "*waving arms furiously*"
    person.fly();

    // 调用的是 Pilot 上定义的 fly 方法
    Pilot::fly(&person);

    // 调用的是 Wizard 上定义的 fly 方法
    Wizard::fly(&person);

    // 完全限定语法 ( Fully Qualified Syntax )
    // 如何调动同名方法
    // 完全限定语法: <Type as Trait>::function(receiver_if_method, netx_args, ...)
    //  - 可以在任何调用函数或方法的地方使用
    //  - 允许忽略那些从其他上下文能推导出来的部分
    //  - 当 Rust 无法区分你期望调用哪个具体实现的时候, 才需要使用这种语法

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // 使用 supertrait 来要求 trait 附带其他 trait 的功能
    // 其实就是要求一个 trait 继承于另一个 trait
    // 需要在一个 trait 中使用其他 trait 的功能
    //  - 需要被依赖的 trait 也被实现
    //  - 那个被间接依赖的 trait 就是当前 trait 的 supertrait

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            // to_string 是由 Display 这个 trait 实现的
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    impl Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}

    let point = Point { x: 1, y: 10 };
    point.outline_print();

    // 使用 newtype 模式在外部类型上实现外部 trait
    // 孤儿规则: 只有当 trait 或类型定义在本地包时, 才能为该类型实现这个 trait
    // 可以通过 newtype 模式来绕过这一规则
    //  - 利用 tuple struct ( 元组结构体 ) 创建一个新的类型
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w)
}
