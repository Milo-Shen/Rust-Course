use std::fmt::Debug;

fn take_ownership(x: String) {
    println!("take ownership: {}", x);
}

fn take_ownership_vec<T: Debug>(x: Vec<T>) {
    println!("take ownership: {:?}", x);
}

fn calculate_length(x: String) -> (String, usize) {
    let length = x.len();
    return (x, length);
}

pub fn learning_ownership() {
    println!("Start to learn ownership");

    // 解引用操作也需要转移所有权
    let v = &vec![11, 22];
    // 因为变量v只是vec的一个引用，而不是它的所有者，它无权转移值的所有权。
    // let vv = *v;
    // 注意，不要使用println!("{}", *a);或类似的宏来测试，这些宏不是函数，它们真实的代码中使用的是&(*a)，因此不会发生所有权的转移。
    println!("{:?}", *v);

    let x = "hello".to_string();
    x; // 发生Move
       // println!("{}", x);  // 报错：value borrowed here after move
       // 从这个示例来看，【当值需要放进位置的时候，就会发生移动】，这句话似乎不总是正确，第三行的x;取得了x的值，但是它直接被丢弃了，所以x也被消耗掉了，使得println中使用x报错。实际上，这里也产生了位置，它等价于let _tmp = x;，即将值移动给了一个临时变量。

    // 从结果上来看，语句块将x通过返回值的方式移出来赋值给了 y，所以认为x的所有权被转移给了y。实际上，语句块中那唯一的一行代码本身就发生了一次移动，将x的所有权移动给了临时变量，然后返回时又发生了一次移动。
    let x = "hello".to_string();
    let y = {
        x // 发生Move，注意没有结尾分号
    };
    // println!("{}", x); // 报错：value borrowed here after move

    let mut name = String::from("jack");
    name.push_str("hello");
    println!("my name is: {}", name);

    let another_str = name.clone();
    println!("my name is: {}", name);

    let ownership_str = String::from("jack");
    take_ownership(ownership_str);
    // print!("{}", ownership_str); value borrowed here after move

    let ownership_str = String::from("jack");
    let (ownership_str_1, length) = calculate_length(ownership_str);
    println!("size of String {} is: {}", ownership_str_1, length);

    let v1 = vec![1, 2, 3];
    take_ownership_vec(v1);
    // error[E0382]: borrow of moved value: `v1`
    // 因为这里 v1 失去了所有权，所以无法再被访问
    // println!("{:?}", v1);

    // Rust 中每个值都有一个所有者，但这个说法比较容易产生误会。
    let s = String::from("hello");

    // 多数人可能会误以为变量s是堆中字符串数据 hello 的所有者，但实际上不是。
    // 前面介绍内存的文章中解释过，String 字符串的实际数据在堆中，但是 String 大小不确定，所以在栈中使用一个胖指针结构来表示这个 String 类型的数据，这个胖指针中的指针指向堆中的 String 实际数据。也就是说，变量 s 的值是那个胖指针，而不是堆中的实际数据。
    // 因此，变量 s 是那个胖指针的所有者，而不是堆中实际数据的所有者。
    // 但是，由于胖指针是指向堆中数据的，多数时候为了简化理解简化描述方式，也经常会说s是那个堆中实际数据的所有者。但无论如何描述，需要理解所有者和值之间的真相。

    // Rust采用非常直接的方式，当执行let s2 = s1;时，直接让s1无效(s1仍然存在，只是变成未初始化变量，Rust不允许使用未初始化变量，可重新为其赋值)，而是只让s2绑定堆内存的数据。也就是将s1移动到s2，也称为值的所有权从s1移给s2。
    let mut s1 = String::from("hello world");
    let s2 = s1;
    s1 = String::from("hello world next");
    println!("{}", s1);

    // 所有权移动后修改数据
    // 定义变量的时候，加上 mut 表示变量可修改。当发生所有权转移时，后拥有所有权的变量也可以加上mut。
    let mut x = String::from("hello");
    // x将所有权转移给y，但 y 无法修改字符串
    let y = x;
    // y.push('C');  // 本行报错
    let a = String::from("hello");
    // 虽然a无法修改字符串，但转移所有权后，b 可修改字符串
    let mut b = a;
    b.push('C'); // 本行不报错

    // 移动真的只是移动吗？
    let s1 = String::from("hello");
    let s2 = s1;

    // 上面已经分析过，值的所有权会从变量s1转移到变量s2，所有权的转移，涉及到的过程是拷贝到目标变量，同时重置原变量到未初始状态，整个过程就像是进行了一次数据的移动。但注意，上面示例中拷贝的是栈中的胖指针，而不是拷贝堆中的实际数据，因此这样的拷贝效率是相对较高的。
    // 所有权转移之后，将只有新的所有者才会指向堆中的实际数据，而原变量将不再指向堆中实际数据，因此所有权转移之后仍然只有一个指针指向堆中数据。
    // Move不仅发生在变量赋值过程中，在函数传参、函数返回数据时也会Move，因此，如果将一个大对象(例如包含很多数据的数组，包含很多字段的struct)作为参数传递给函数，是否会让效率很低下？
    // 按照上面的结论来说，确实如此。但Rust编译器会对Move语义的行为做出一些优化，简单来说，当数据量较大且不会引起程序正确性问题时，它会传递大对象的指针而非内存拷贝。
    // 此外，对于胖指针类型的变量(如Vec、String)，即使发生了拷贝，其性能也不差，因为拷贝的只是它的胖指针部分。
    // 总之，Move虽然发生了内存拷贝，但它的性能并不会太受影响。
    // 此处部分结论参考：https://stackoverflow.com/questions/30288782/what-are-move-semantics-in-rust。

    // Copy语义
    // 默认情况下，在将一个值保存到某个位置时总是进行值的移动(实际上是拷贝)，使得只有目标位置才拥有这个值，而原始变量将变回未初始化状态，也就是暂时不可用的状态。这是Rust的移动语义。
    // Rust 还有 Copy 语义，和 Move 语义几乎相同，唯一的区别是 Copy 后，原始变量仍然可用。
    // 前面说过，Move 实际上是进行了拷贝，只不过拷贝后让原始变量变回未初始化状态了，而 Copy 的行为，就是保留原始变量。
    // 但 Rust 默认是使用 Move 语义，如果想要使用 Copy 语义，要求要拷贝的数据类型实现了 Copy Trait。
    // 例如，i32 默认就已经实现了Copy Trait，因此它在进行所有权转移的时候，会自动使用 Copy 语义，而不是 Move 语义。

    let x = 3; // 3是原始数据类型，它直接存储在栈中，所以x变量的值是3，x拥有3
    let n = x; // Copy x的值(即3)到变量n，n现在拥有一个3，但x仍然拥有自己的3

    // Rust中默认实现了Copy Trait的类型，包括但不限于：
    // 1. 所有整数类型，比如u32
    // 2. 所有浮点数类型，比如f64
    // 3. 布尔类型，bool，它的值是true和false
    // 4. 字符类型，char
    // 5. 元组，当且仅当其包含的类型也都是Copy的时候。比如(i32, i32)是Copy的，但(i32, String)不是
    // 6. 共享指针类型或共享引用类型

    // 对于那些没有实现 Copy 的自定义类型，可以手动去实现 Copy (要求同时实现 Clone )，方式很简单：
    #[derive(Copy, Clone)]
    struct Abc(i32, i32);

    // 下面是实现了 Copy 和未实现 Copy 时的一个对比示例：
    #[derive(Debug)]
    struct Xyz(i32, i32);

    #[derive(Copy, Clone, Debug)]
    struct Def(i32, i32);

    fn main() {
        let x = Xyz(11, 22);
        let y = x;
        // println!("x: {:?}", x); // 报错
        println!("y: {:?}", y);

        let d = Def(33, 44);
        let e = d;
        println!("d: {:?}", d);
        println!("e: {:?}", e);
    }
}
