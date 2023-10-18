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
}
