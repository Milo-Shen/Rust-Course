use std::fmt::Display;

pub fn learning_lifecycle() {
    println!("Start to learn life cycle - 4");

    // 方法中的生命周期标注
    // 在 struct 上使用生命周期实现方法，语法和泛型参数的语法一样
    // 在哪生命和使用生命周期参数，依赖于：生命周期参数是否和字段，方法的参数和返回值有关

    // struct 字段的生命周期名:
    //  - 在 impl 后生命
    //  - 在 struct 名后使用
    //  - 这些生命周期是 struct 类型的一部分

    // impl 块内的方法签名中:
    //  - 引用必须绑定于 struct 字段引用的生命周期，或者引用是独立的也可以
    //  - 生命周期省略规则经常使得方法中的生命周期标注不是必须的

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        // 因为这边返回值不是引用，所以无需为返回值申明生命周期
        fn level(&self) -> i32 {
            1
        }

        // 根据省略规则 1, 编译器会为每个引用申明生命周期
        // 根据省略规则 2, 返回值的生命周期即为 self 的生命周期
        // 所以下面的这个方法不需要显式申明生命周期
        fn announce(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // 静态生命周期
    // 'static 是一个特殊的生命周期: 整个程序的持续时间
    // 但是使用 'static 生命周期需要思考: 是否需要引用在程序整个生命周期内都存活
    //  - 例如: 所有的字符串字面值都拥有 'static 生命周期
    let s: &'static str = "I have a static lifetime";

    // 泛型参数类型, Trait Bound, 生命周期
    fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string_1 = "Hello World";
    let string_2 = "I am the longest string";
    let longest_string = longest_with_announcement(&string_1, &string_2, "I am demo");
    println!("The longest string is: {}", longest_string);
}
