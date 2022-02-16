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
        shoes.into_iter().filter(|x| x.size == shoe_size).collect()
    }
}