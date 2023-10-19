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
}
