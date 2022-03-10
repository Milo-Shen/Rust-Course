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

    // 使用类型别名后
    type Thunk = Box<dyn Fn() + Send + 'static>;

    fn takes_long_type(f: Thunk) {}
    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hello world"))
    }
    let f: Thunk = Box::new(|| println!("hello world"));
}