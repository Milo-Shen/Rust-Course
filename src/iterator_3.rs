pub fn learning_iterator() {
    println!("Start to learn iterator 3");

    // 使用闭包来捕获环境
    // filter 方法: ( 其实迭代器适配器，用它来演示闭包捕获环境的常见用法 )
    //  - 接收一个闭包
    //  - 这个闭包在遍历迭代器的每个元素时，会返回一个 bool 类型
    //  - 如果闭包返回 true: 当前元素将会包含在 filter 产生的迭代器中
    //  - 如果闭包返回 false: 当前元素将不会包含在 filter 产生的迭代器中
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // 此处的 into_iter 会获取 shoes 的所有权
        // todo: 这里的 == 使用的是默认实现，思考如何采用自己的实现
        // 此处采用 into_iter 的原因是，若是采用 iter，则 x 的类型为 &&Shoe, 使用 iter 时候的类型才是 &shoe
        shoes.into_iter().filter(|x| x.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 15, style: String::from("sandal") },
        Shoe { size: 20, style: String::from("boot") },
    ];

    let my_size_shoes = shoes_in_my_size(shoes, 10);
    println!("{:?}", my_size_shoes);
}