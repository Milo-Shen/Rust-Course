use std::clone::Clone;
use std::fmt::{Debug, Display};

trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    // 对于 trait 的默认实现
    fn print(&self) {
        println!(
            "this is the default implementation of trait summary, from: {}",
            self.summarize_author()
        );
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// 在类型上实现 trait
// impl xxx for Tweet {...}
// 在 impl 的块里，需要对 Trait 里的方法签名进行具体的实现
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.content);
    }

    fn summarize_author(&self) -> String {
        "Milo-Shen".to_string()
    }

    // 需要注意的是：无法从方法重写的实现里面调用默认的实现
    fn print(&self) {
        println!("customized implementation of NewsArticle");
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
    fn summarize_author(&self) -> String {
        "Milo-Shen".to_string()
    }
}

pub fn learning_trait() {
    println!("Start to learn trait");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("trait_demo_tweet"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    // 实现 trait 的约束
    // 可以在某个类型上实现 trait 的前提条件是：这个类型或这个 trait 是在本地 crate 里定义的
    // 无法为外部类型实现外部的 trait，这个限制是程序属性的一部分（也就是一致性）
    // 也称为 "孤儿类型" ，之所以这样命名是因为父类型不存在。如果没有这个规则，则 2 个 crate 可以为同一个类型
    // 实现用一个 trait ，rust 就不知道应该使用哪个实现了

    // 使用 trait 的默认实现
    tweet.print();

    // 使用默认实现的一个重写的实现
    let news = NewsArticle {
        headline: "Shanghai News".to_string(),
        location: "China".to_string(),
        author: "Milo-Shen".to_string(),
        content: "Demo".to_string(),
    };
    news.print();

    // 将 trait 作为参数 - impl Trait 写法，适用于简单的情况
    // 该语法也是下面
    pub fn notify_impl(item: &impl Summary) {
        println!("Breaking news ! {}", item.summarize());
    }
    notify_impl(&tweet);
    notify_impl(&news);

    // Trait bound 语法，可用于复杂的情况
    pub fn notify_bound<T: Summary>(item: &T) {
        println!("Breaking news ! {}", item.summarize());
    }
    notify_bound(&tweet);
    notify_bound(&news);

    // 使用 + 指定多个 Trait bound
    pub fn notify_impl_display(item: &(impl Summary + Display)) {
        println!("Breaking news ! {}", item.summarize());
    }

    pub fn notify_bound_display<T: Summary + Display>(item: &T) {
        println!("Breaking news ! {}", item.summarize());
    }

    // Trait bound 使用 where 子句
    // 未使用 where 子句
    pub fn notify_bound_without_where<T: Summary + Display, U: Clone + Debug>(
        a: T,
        b: U,
    ) -> String {
        return format!("Breaking news ! {}", a.summarize());
    }
    // 使用 where 子句 - 可以在方法签名的后面指定 where 子句
    pub fn notify_bound_with_where<T, U>(a: T, b: U) -> String
    where
        T: Summary + Display,
        U: Clone + Debug,
    {
        return format!("Breaking news ! {}", a.summarize());
    }

    // 实现 Trait 作为返回类型 - 使用 impl 的方式
    // 使用 impl Trait 只能返回确定的同一种类型，返回可能不同类型的代码会报错
    pub fn return_trait_impl() -> impl Summary {
        return NewsArticle {
            headline: "Shanghai News".to_string(),
            location: "China".to_string(),
            author: "Milo-Shen".to_string(),
            content: "Demo".to_string(),
        };
    }
    let return_impl_result = return_trait_impl();
    return_impl_result.summarize();

    // 实现 Trait 作为返回类型 - 使用 bound 的方式
    // todo: 这里研究一下 ist.iter() 的用法
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for item in list {
            if *item > largest {
                largest = *item;
            }
        }
        return largest;
    }
    let vector = vec![1, 2, 3];
    let largest = largest(&vector);
    println!("The largest value is: {}", largest);

    fn largest_string_1<T: PartialOrd + Clone>(list: &[T]) -> T {
        let mut largest = list[0].clone();
        for item in list {
            if *item > largest {
                largest = (*item).clone();
            }
        }
        return largest;
    }
    let vector = vec![String::from("a"), String::from("b")];
    let largest = largest_string_1(&vector);
    println!("The largest value is: {}", largest);

    fn largest_string_2<T: PartialOrd + Clone>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            // todo: 判断 (*item) > (*largest) 与 item > largest 的异同
            if (*item) > (*largest) {
                largest = item;
            }
        }
        return largest;
    }
    let vector = vec![String::from("a"), String::from("b")];
    let largest = largest_string_2(&vector);
    println!("The largest value is: {}", largest);

    let a = 2;
    let b = &a;
    let c = 3;
    let d = a * c;
    println!("The result is: {}", d);

    // 使用 Trait bound 有条件的实现方法
    // 在使用泛型类型参数的 impl 块上使用 Trait bound，我们可以有条件得为实现了特定 Trait 的类型来实现方法
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest number is x = {}", self.x);
            } else {
                println!("The largest number is y = {}", self.y);
            }
        }
    }

    let num_pair = Pair { x: 1, y: 1 };
    num_pair.cmp_display();

    // 也可以为实现了其他 Trait 的任意类型有条件的实现某个 Trait
    // 为满足 Trait Bound 的所有类型上实现 Trait 叫做覆盖实现 ( blanket implementations )
    trait ToString {
        fn to_string(&self) -> String;
    }
    impl<T: Display> ToString for T {
        fn to_string(&self) -> String {
            todo!("to implement to_String method")
        }
    }
}
