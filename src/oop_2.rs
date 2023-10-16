pub trait Draw {
    fn draw(&self);
}

pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Screen {
    // dyn Draw 语法的含义是: Box 里面的元素都实现了 Draw 这个 trait
    // 这边没有使用泛型的原因是, 使用泛型的话一次只能使用一个类型, 而 dyn Draw 的话
    // 只要实现了 Draw 这个的 trait 都能放在 components 这个 vec 里面
    pub components: Vec<Box<dyn Draw>>,
    // 下面的代码会报错, 因为返回的类型是 Self
    // error[E0038]: the trait `oop_2::Clone` cannot be made into an object
    // because method `clone` references the `Self` type in its return
    // pub components: Vec<Box<dyn Clone>>,
}

impl Screen {
    pub fn run(&self) {
        // draw 函数运行的时候, 它不关心传入的是什么类型, 它只关心传入的的类型实现了 Draw 这个 trait 就可以
        // 但是如果元素没有实现 Draw 这个 trait 则不行
        for component in self.components.iter() {
            // 此处 component 的类型为: &Box<dyn Draw>
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw a Button");
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw a SelectBox");
    }
}

pub fn learning_oop() {
    println!("Start to learn oop 2");

    // 使用 trait 对象来存储不同类型的值
    // 有这样一个需求
    // 创建一个 GUI 工具:
    //  - 它会遍历某个元素的列表, 依次调用元素的 draw 方法进行绘制
    //  - 例如: Button、TextField 等元素

    // 在面向对象语言里:
    //  - 定义一个 Component 父类, 里面继承了 draw 方法
    //  - 定义 Button、TextField 等类, 继承于 Component 类

    // 为共有行为定义一个 trait
    // Rust 避免将 struct 与 enum 称为对象, 因为它们与 impl 块是分开的
    // trait 对象有些类似于其他语言中的对象:
    //  - 它们某种程度上组合了数据与行为
    // trait 对象与传统对象不同的地方:
    //  - 无法为 trait 对象添加数据
    // trait 对象被专门用于抽象某些共有行为, 它没有其他语言中的对象那么通用

    // Trait 对象执行的是动态派发
    // 将 trait 约束作用于泛型时, Rust 编译器会执行单态化:
    //  - 编译器会为我们用来替换泛型类型参数的每一个具体类型生成对应函数和方法的非泛型实现

    // 通过单态化生成的代码会执行静态派发 ( static dispatch ), 在编译过程中确定调用的具体方法

    // 动态派发 ( dynamic dispatch ):
    //  - 无法在编译过程中确定你调用的究竟是哪一种方法
    //  - 编译器会产生额外的代码以便在运行时找出希望调用的方法

    // 使用 trait 对象, 会执行动态派发:
    //  - 产生运行时开销
    //  - 阻止编译器内联方法代码, 使得部分优化操作无法进行

    // Trait 对象必须保证对象安全
    // 只能把满足对象安全 ( object-safe ) 的 trait 转化为 trait 对象
    // Rust 采用一系列规则来判定某个对象是否安全, 只需要记住两条:
    //  - 方法中的返回类型不是 Self
    //  - 方法中不包含任何泛型类型参数

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
        ],
    };

    screen.run();
}
