pub fn learning_iterator() {
    println!("Start to learn iterator");

    // 什么是迭代器
    // 迭代器模式: 对一系列项执行执行某些任务
    // 迭代器负责:
    //  - 遍历每一个项
    //  - 确定序列 ( 遍历 ) 何时完成
    // Rust 的迭代器
    //  - 惰性的: 除非调用消费迭代器的方法，否则迭代器本身没有任何效果
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // 此处的 val 是 &i32 类型 ( 是一个借用 )
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // 此处的 val_1 是 i32 类型
    for val_1 in v1 {
        println!("Got: {}", val_1);
    }

    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_into_iter = v1.into_iter();

    // 此处的 val 是 i32 类型 ( 是一个借用 )
    for val in v1_into_iter {
        println!("Got: {}", val);
    }

    let mut v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter_mut = v1.iter_mut();

    // 此处的 val 是 &mut i32 类型 ( 是一个借用 )
    for val in v1_iter_mut {
        println!("Got: {}", val);
    }

    // 迭代器 (1) - Iterator trait 和 next 方法
    // Iterator trait
    // 所有的迭代器都实现了 Iterator trait
    // Iterator trait 定义于标准库，定义大致如下: ( 加 _ 是为了和系统的 Iterator 作区分 )
    pub trait _Iterator {
        // type Item 和 Self::Item 定义了与此 trait 关联的类型
        //  - 实现 Iterator trait 需要你定义一个 Item 类型，它用于 next 方法的返回类型 ( 迭代器的返回类型 )
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    // Iterator 的几个迭代方法
    //  - iter 方法       : 在不可变引用上创建迭代器
    //  - into_iter 方法  : 创建的迭代器会获得所有权
    //  - iter_mut 方法   : 迭代可变的引用

    //  使用 iter 方法 - 在不可变引用上创建迭代器
    let v2 = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let mut v2_iter = v2.iter();
    // 我们可以看到，v3 仍旧拥有所有权，iter_mut 不会取得所有权
    println!("v2: {:?}", v2);
    let v2_iter_first = v2_iter.next().unwrap();
    // error[E0596]: cannot borrow `*v1_iter_first` as mutable, as it is behind a `&` reference
    // `v1_iter_first` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // 因为 iter 返回的是在不可变引用上创建的迭代器，所以不能改变迭代器 next 的返回值
    // v1_iter_first.push_str("is mut ?");

    //  使用 iter_mut 方法 - 迭代可变的引用
    let mut v3 = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let mut v3_iter = v3.iter_mut();
    let v3_iter_first = v3_iter.next().unwrap();
    v3_iter_first.push_str("is mut ?");
    // todo: 45, 46 行代码反过来就会报错，思考我什么 ?
    println!("v3_iter_first is: {}", v3_iter_first);
    println!("The first item of vec is: {}", v3.get(0).unwrap());
    // 我们可以看到，v3 仍旧拥有所有权，iter_mut 不会取得所有权
    println!("v3: {:?}", v3);

    // 使用 into_iter 方法 - 创建的迭代器会获得所有权
    let mut v4 = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let mut v4_iter = v4.into_iter();
    let v4_iter_first = v4_iter.next().unwrap();
    println!("v4_iter_first is: {}", v4_iter_first);
    // borrow of moved value: `v4`
    // into_iter 创建的迭代器获取了 v4 的所有权，所以 v4 在下面的 println 中无法访问
    // println!("v4: {:?}", v4);
}

// 故而 Iterator trait 仅要求实现一个方法: next
// next:
//  - 每次返回迭代器中的一项
//  - 返回结果包裹在 Some 里
//  - 迭代结束，返回 None
// 可直接在迭代器上调用 next 方法
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1: Vec<i32> = vec![1, 2, 3];
        // 在调用 next 方法时, 会改变迭代器中记录序列位置的变量的值，所以此处要加 mut
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }
}
