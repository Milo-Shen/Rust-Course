pub fn learning_iterator() {
    println!("Start to learn iterator 4");

    // 创建自定义迭代器
    // 使用 Iterator trait 来创建自定义迭代器 ( 也就是实现 next 方法 )

    struct Counter {
        count: u32,
    }

    impl Counter {
        // 该函数是一个关联函数
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();
    let first: Option<u32> = counter.next();
    match first {
        Some(t) => println!("{}", t),
        None => println!("None"),
    }
    // todo: 为什么用 for in 的时候，value 的类型会自动解构成 u32 ?
    for value in counter {
        println!("{:?}", value);
    }

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("The sum value is: {}", sum);
}
