use std::slice;

// 这一类型的 extern 功能, 不需要使用 unsafe
#[no_mangle]
pub extern "C" fn call_from_C() {
    println!("Just called a Rust function from C!");
}

// 申明一个全局变量, 全局变量推荐使用大写的形式
// 生命周期: 静态变量只能存储拥有 'static 这个生命周期的引用
static HELLO_WORLD: &str = "Hello World";

// 必须标注类型, 若是不标注的话, 编译器会报错
// error: missing type for `static` item
// 定义一个不可变的静态变量
static RATIO: f64 = 3.14;
// 定义一个可变的静态变量
static mut PI: f64 = 3.14;

pub fn learning_unsafe_rust() {
    println!("start to learn unsafe rust");

    // 不安全 Rust
    // 匹配命名变量
    // 隐藏着第二个语言, 它没有强制内存安全保证: unsafe rust ( 不安全的 rust )
    //  - 和普通 rust 一样, 但提供了额外的超能力

    // unsafe rust 存在的原因
    //  - 静态分析是保守的
    //    - 使用 unsafe rust: 我知道自己在做什么, 并承担相应的风险
    //  计算机硬件本身就是不安全的, rust 需要能够进行底层系统编程

    // unsafe 超能力
    // 使用 unsafe 关键字切换到 unsafe rust, 开启一个块, 里面放着 unsafe 代码
    // unsafe rust 里可执行的四个动作 ( unsafe 超能力 )
    //  - 解引用原始指针
    //  - 调用 unsafe 函数或方法
    //  - 访问或修改可变的静态变量
    //  - 实现 unsafe trait

    // 注意:
    // unsafe 并没有关闭借用检查或停用其他安全检查
    // 任何内存安全的错误必须停留在 unsafe 块里
    // 尽可能隔离 unsafe 代码, 最好将其封装在安全的抽象里, 提供安全的 API

    // 解引用原始指针 ( raw pointer )
    // 原始指针
    //  - 可变的: *mut T
    //  - 不可变的: *const T  意味着指针在解引用后不能直接对其进行赋值
    //  - 这里的 * 不是解引用符号, 它是类型名的一部分

    // 与引用不同, 原始指针:
    //  - 我们先来复习一下借用规则: 多个指向同一个区域的可变引用会导致数据的竞争，以及数据的不一致
    //  - 借用规则: 在任何给定的时间, 你要么只能拥有一个可变引用, 要么只能拥有任意数量的不可变引用
    //  - 允许通过同时具有不可变和可变指针或多个指向同一位置的可变指针来忽略借用规则
    //  - 无法保证能指向合理的内存
    //  - 允许为 null
    //  - 不实现任何的自动清理
    // 放弃保证的安全, 换取更好的性能 / 与其他语言或硬件接口的能力

    // 我们可以在不安全代码块之外创建原始指针, 但是只能在不安全代码块里面对其进行解引用
    let mut num = 5;

    // 我们分别将不可变和可变引用转变成了原始指针 *const 和 *mut , 因为这 2 个指针是来自有效的引用
    // 所以我们知道这 2 个指针也是有效的
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 原始指针并不一直都是有效的, 下面我们创建一个无法确定其有效性的原始指针
    let address = 0x012345usize;
    let r = address as *const i32;

    // 解引用原始指针
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    // r1 和 r2 指向了同一块内存地址, 此时我们可以通过可变引用 r2 来修改其里面的值
    // 这种操作是允许的, 但是需要格外小心
    unsafe {
        *r2 = 10000;
        println!("updated r2: {}", *r2);
        // 因为上面我们通过可变的引用改变了 r2 的值, r2 和 r1 又指向同一片内存空间
        // 所以下面的输出结果是: 10000
        println!("updated r1: {}", *r1);
    }

    // 为什么要用原始指针
    //  - 与 C 语言进行接口
    //  - 构建借用检查器无法理解的安全抽象

    // 调用 unsafe 函数或方法
    // unsafe 函数或方法: 在定义前加上了 unsafe 关键字
    //  - 调用前需手动满足一些条件 ( 主要看文档 ), 因为 Rust 无法对这些条件进行验证
    //  - 需要在 unsafe 块里进行调用
    unsafe fn dangerous() {
        println!("This is a unsafe rust function")
    }
    // error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
    // unsafe 的方法必须放在 unsafe 代码块里调用, 否则编译器会报错
    // dangerous();

    unsafe {
        dangerous();
    }

    // 创建 unsafe 代码的安全抽象
    // 函数包含 unsafe 代码并不意味着需要将整个函数标记为 unsafe
    // 将 unsafe 代码包裹在安全函数中是一个常见的抽象
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let r: &mut [i32] = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    println!("a = {:?}, b = {:?}", a, b);

    fn _split_at_mut(slice: &mut [i32], mid: usize) {
        let len = slice.len();
        assert!(mid <= len);
        // error[E0499]: cannot borrow `*slice` as mutable more than once at a time
        // 下面这句话会报错, 因为可变借用多于 1 个, 这违反了借用规则
        // (&mut slice[..mid], &mut slice[mid..])
    }

    // split_at_mut 是一个不安全代码的安全抽象
    // 因为函数内部调用了 unsafe 的代码, 但是函数体本身不是 unsafe 的
    // 所以我们就可以在安全的代码中调用 split_at_mut 了
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr: *mut i32 = slice.as_mut_ptr();
        assert!(mid <= len);
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid)
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    // 使用 extern 函数调用外部代码
    // extern 关键字: 简化创建和使用外部函数接口 ( FFI ) 的过程
    // 外部函数接口 ( FFI, Foreign Function Interface ): 它允许一种编程语言定义函数, 并让其他编程语言调用这些函数
    // 任何在 extern 内申明的函数都是不安全的
    // 应用二进制接口 ( ABI, Application Binary Interface ): 定义函数在汇编层的调用方式
    // "C" ABI 是最常见的 ABI, 它遵循 C 语言的 ABI

    // 从其他语言调用 Rust 函数
    // 可以使用 extern 创建接口, 其他语言通过它们可以调用 Rust 的函数
    // 在 fn 前添加 extern 关键字, 并指定 ABI
    // 还需要添加 #[no_mangle] 注解: 避免 Rust 在编译时改变它的名称

    // 访问或修改一个可变静态变量
    // Rust 支持全局变量, 但因为所有权机制可能产生某些问题, 例如数据竞争
    // 在 Rust 里, 全局变量叫做静态 ( static ) 变量
    println!("The static variable is = {}", HELLO_WORLD);

    // 静态变量
    // 静态变量与常量类似
    // 命名: SCREAMING_SNAKE_CASE
    // 必须标注类型
    // 静态变量只能存储 'static 生命周期的引用, 无需显式引用
    // 访问不可变的静态变量是安全的

    // 常量和不可变静态变量的区别
    // 静态变量: 有固定的内存地址, 使用它的值总会访问同样的数据
    // 常量: 允许使用它们的时候对数据进行复制
    // 静态变量： 可以是可变的, 访问和修改静态可变变量是不安全 ( unsafe ) 的

    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    // error[E0133]: use of mutable static is unsafe and requires unsafe function or block
    // 访问和修改静态可变变量是不安全, 所以下面访问可变静态变量的操作应该放在 unsafe 块里面
    // println!("COUNT: {}", COUNTER);
    unsafe {
        println!("COUNT: {}", COUNTER);
    }

    // 访问一个不可变静态变量则不需要放在 unsafe 代码块中
    println!("{}", HELLO_WORLD);

    // 实现不安全 ( unsafe ) trait
    // 当某个 trait 中存在至少一个方法拥有编译器无法校验的不安全因素时, 就称这个 trait 是不安全的
    // 声明 unsafe trait: 定义前加 unsafe 关键字
    //  - 该 trait 只能在 unsafe 代码块中实现
    unsafe trait Foo {}
    struct UnsafeExample {}
    unsafe impl Foo for UnsafeExample {}

    // 何时使用 unsafe 代码
    // 编译器无法保证内存安全, 保证 unsafe 代码正确并不简单
    // 有充足理由使用 unsafe 代码时, 就可以这样做
    // 通过显式标记 unsafe, 可以在出现问题时轻松地定位
}