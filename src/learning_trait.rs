use std::os::unix::raw::ino_t;

trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    // 对于 trait 的默认实现
    fn print(&self) {
        println!("this is the default implementation of trait summary, from: {}", self.summarize_author());
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
}