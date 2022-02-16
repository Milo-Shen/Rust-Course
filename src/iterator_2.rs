pub fn learning_iterator() {
    println!("Start to learn iterator 2");

    // 消耗和产生迭代器

    // 消耗迭代器的方法
    // 在标准库中，Iterator trait 有一些带默认实现的方法
    // 其中有一些方法会调用 next 方法
    //  - 实现 Iterator trait 时必须实现 next 方法的原因之一
    // 调用 next 的方法叫做 "消耗型适配器"
    //  - 因为调用它们会把迭代器消耗尽
    // 例如: sum 方法 ( 该方法会耗尽迭代器 )
    //  - 取得迭代器的所有权
    //  - 通过反复调用 next, 遍历所有元素
    //  - 每次迭代，把当前元素添加到一个总和里，迭代结束，返回总和
    let v1: Vec<u32> = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: u32 = v1_iter.sum();
    println!("The total value of v1 is: {:?}", total);
    // error[E0382]: borrow of moved value: `v1_iter`
    // 因为 sum 会取得迭代器的所有权，所以此处无法再访问 v1_iter
    // println!("The v1 iter is: {:?}", v1_iter);

    // 产生其他迭代器的方法
    // 定义在 Iterator trait 上的另外一些方法叫做 "迭代器适配器"
    //  - 把迭代器转换为不同种类的迭代器
    // 可以通过链式调用使用多个迭代器适配器来执行复杂的操作，这种调用可读性高
    // 例如: map
    //  - 接收一个闭包作为参数，闭包作用于每个元素
    //  - 产生一个新的迭代器
    let v1 = vec![1, 2, 3];
    // 因为迭代器是惰性的, 除非调用消费迭代器的方法, 否则迭代器本身没有任何效果
    // collect 是一个消耗型迭代器, 它会把结果收集到一个集合中
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
}