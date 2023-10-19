const VERSION: &str = "1.0.0";

pub fn learning_variables() {
    // Variables and Mutability
    println!("Start to learn variables and mutability");
    // a address is :0x140701963303620
    let a: i32 = 5;
    let address = &a as *const i32 as usize;
    println!("a address is :0x{}", address);

    // a address is :0x140701963303708
    let a: i32 = 10;
    let address = &a as *const i32 as usize;
    println!("a address is :0x{}", address);

    // change the type of a by shadow mechanism
    let a: String = String::from("shadow");
    println!("the value of a = {}", a);

    // b address is :0x140701963303796
    let mut b: i32 = 5;
    let address = &b as *const i32 as usize;
    println!("b address is :0x{}", address);

    // b address is :0x140701963303796
    // rust don't update the address of variable b if we use mut syntax
    b = 50;
    let address = &b as *const i32 as usize;
    println!("b address is :0x{}", address);

    {
        let s = "hello";
        // 0x10257459c
        println!("the address of const s is: {:p}", s);
    }

    // 因此，上面的示例中只是让变量s失效了，仅此而已，并没有销毁s所绑定的字符串字面量
    let s = "hello";
    // 0x10257459c
    println!("the address of const s is: {:p}", s);

    let tuple: (i32, f64) = (1, 2.0);
    let address = &tuple as *const (i32, f64) as usize;
    println!("my tuple is: {}, {}", tuple.0, tuple.1);
    println!("my tuple address is: {}", address);

    let (_a, _b) = tuple;
    println!("my tuple is: {}, {}", _a, _b);

    let array_a: [i32; 2] = [1, 2];
    let array_b = [3; 2];
    let mut array_c = [1, 2];
    println!("array value: {}, {}, {:?}", array_a[0], array_b[0], array_c);
    println!("array len of array_b is: {}", array_b.len());

    // char 和 u8 的区别
    let a = 'A';
    let b = b'A';
    println!("{}, {}", a, b);

    // Rust 提供了一个名为 Copy 的 trait, 它可以用于整数这类完全存储在栈上的数据类型。
    // 一旦某种数据类型拥有了 Copy 这种 trait, 那么它的变量就可以在赋值给其他变量之后保持可用性。
    // 如果一种类型本身或这种类型的任意成员实现了 Drop 这种 trait, 那么 Rust 就不允许其实现 Copy 这种 trait
    // 尝试给某个需要在离开作用域时执行特殊指令的类型实现 Copy 这种 trait 会导致编译时错误
}
