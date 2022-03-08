use std::ops::Add;

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
        // 此处的 Self::Output 也可以由 Point 来代替
        fn add(self, rhs: Self) -> Self::Output {
            Self::Output {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    let my_pointer = Point { x: 1, y: 2 } + Point { x: 3, y: 4 };
    println!("The final point is: {:?}", my_pointer);
}