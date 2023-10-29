use std::fmt;
use std::io::Error;

pub fn learning_advanced_type() {
    println!("Start to learn advanced type");

    // 使用 newtype 模式实现类型的安全和抽象
    // newtype 模式可以:
    //  - 用来静态的保证各种值之间不会混淆并表明值的单位
    //  - 为类型的某些细节提供抽象能力
    //  - 通过轻量级的封装来隐藏内部实现的细节

    // 使用类型的别名创建类型的同义词
    // Rust 提供了类型别名的功能:
    //  - 为现有类型生成另外的名称
    //  - 并不是一个独立的类型
    //  - 使用 type 关键字

    // 主要用途: 减少代码字符重复
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    // 未使用类型别名前:
    fn _takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
    fn _returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        Box::new(|| println!("hello world"))
    }
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hello world"));

    // 使用类型别名后:
    type Thunk = Box<dyn Fn() + Send + 'static>;

    fn takes_long_type(f: Thunk) {}
    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hello world"))
    }
    let f: Thunk = Box::new(|| println!("hello world"));

    // Result 的例子
    // 未使用类型别名前:
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn flush(&mut self) -> Result<usize, Error>;
        fn write_all(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<usize, Error>;
    }

    // 使用类型别名后:
    type _Result<T> = Result<T, std::io::Error>;
    pub trait _Write {
        fn write(&mut self, buf: &[u8]) -> _Result<usize>;
        fn flush(&mut self) -> _Result<usize>;
        fn write_all(&mut self, buf: &[u8]) -> _Result<usize>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> _Result<usize>;
    }

    // 使用类型别名后 - 另一种形式:
    type __Result<T> = std::io::Result<T>;
    pub trait __Write {
        fn write(&mut self, buf: &[u8]) -> __Result<usize>;
        fn flush(&mut self) -> __Result<usize>;
        fn write_all(&mut self, buf: &[u8]) -> __Result<usize>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> __Result<usize>;
    }

    // Never 类型
    // 有一个名为 ! 的特殊类型
    //  - 它没有任何值, 行话称为空类型 ( empty type )
    //  - 我们倾向于叫它 never 类型, 因为它在不返回的函数中充当返回类型
    // 不返回值的函数也被称为发散函数 ( diverging function )
    // Never 这个类型无法产生一个无法返回的值, Never 类型的表达式可以被强制转换为其他任意类型
    // 永远不会结束的 loop 循环的表达式的返回值也是 Never 类型
    // continue 和 panic! 返回的类型就是 Never 类型
    // fn bar() -> ! {}

    let guess = "2";
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // continue 就是 Never 类型, 而 Never 这个类型无法产生一个可供返回的值
            // 因而下方的表达式的返回类型就采用了第一个分支的类型, 也就是 u32 类型
            // 因为 Never 类型的表达式可以强制被转换成其他任意类型, 所以此处 continue 所代表的 Never 就被强制转化为了 u32 类型
            // 所以上下这两句话的返回类型是一样的, 都是 u32 类型
            Err(_) => continue,
        };
        println!("guess: {:?}", guess);
        break;
    }

    // 动态大小和 Sized Trait
    // Rust 需要在编译时确定为一个特定类型的值分配多少空间
    // 动态大小的类型 ( Dynamically Sized Types, DST ) 概念
    //  - 编写代码时只有在运行时才能确定大小的值
    // str 是动态大小的类型 ( 注意不是 &str ): 只有运行时才能确定字符串的长度
    //  - 下面的代码无法工作:
    //    - let s1: str = "Hello there !"
    //    - let s2: str = "How's it going ?"
    //  - 对于同一个类型, Rust 会为其分配同样大小的内存空间 ( 同一个类型的值必须使用等量的内存 ), 所以上面的写法是错误的
    //  - 使用 &str 来解决
    //    - str 的地址
    //    - str 的长度

    // Rust 使用动态大小类型的通用方式
    // 附带一些额外的元数据来存储动态信息的大小
    //  - 使用动态大小类型时总会把它的值放在某种指针的

    // 另一种动态大小的类型: Trait
    // 每个 trait 都是一个动态大小的类型, 可以通过名称对其进行引用
    // 为了将 trait 用作 trait 对象, 必须将其放置在某种指针之后
    //  - 例如: &dyn Trait 或 Box<dyn Trait> 或 Rc<dyn Trait> 之后

    // Sized Trait
    // 为了处理动态大小的类型, Rust 提供了一个 Sized Trait 来确定一个类型的大小在编译时是否已知
    //  - 编译时可计算出大小的类型会自动实现这一 trait
    //  - Rust 还会为每一个泛型函数隐式地添加 Sized 约束
    // 默认情况下， 泛型函数只能被用于编译时已经知道大小的类型, 但是可以通过特殊语法解除这一限制

    // 下面的函数实际上会被影式转化为: fn generic<T: Sized>(t: T) {}
    fn generic<T>(t: T) {}

    // 使用 ?Sized trait 解除上述约束 ( 所以 ?Sized 是专门针对泛型设计的 )
    // ?Sized 表达了一种不确定性, 即 T 可能是 sized 也可能不是 sized
    // 语法的要求: 只能用于 Sized 上面
    // 参数 T 变成了 &T, 因为此时 T 的大小不是确定的, 所以此处我们要把 T 放在某种指针后面 ( 这个例子里使用的是引用 )
    fn _generic<T: ?Sized>(t: &T) {}
}
