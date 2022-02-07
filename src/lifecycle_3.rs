pub fn learning_lifecycle() {
    println!("Start to learn life cycle - 3");

    // 深入理解生命周期
    // 1. 指定生命周期参数的方式依赖于函数所做的事情

    // 在这个例子里面，函数返回值的生命周期只和 x 有关，所以对于形参 y 的生命周期描述可以去掉
    fn longest_1<'a>(x: &'a str, y: &str) -> &'a str {
        return x;
    }

    // 从函数返回引用时，返回类型的生命周期参数需要与其中一个参数的生命周期匹配
    // 如果返回的引用没有指向任何参数，那么它只能引用函数内创建的值，这就会造成悬垂引用
    // "悬垂引用"：该值在函数结束时就走出了作用域
    fn longest_2<'a>(x: &'a str, y: &str) -> &'a str {
        let result = String::from("悬垂引用");
        // error[E0515]: cannot return reference to local variable `result`
        // returns a reference to data owned by the current function
        // Rust 不允许悬垂引用，所以下面的代码会报错
        // return result.as_str();
        return x;
    }

    fn longest_3(x: &str, y: &str) -> String {
        // 若想把函数体内的值传出函数体外，可以直接返回值本身而不是返回引用
        // 该操作会把返回值的所有权移交给函数的调用者，由函数的调用者来清理内存
        return String::from("悬垂引用");
    }

    // Struct 定义中的生命周期标注
    // Struct 里可以包括：
    //  - 自持有的类型， i32 这种基本类型称为自持有类型
    //  - 引用：需要在每个引用上添加生命周期标注
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Jack. Some years ago");
    let first_sentence = novel.split('.').next().expect("could not found '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The first sentence is: {}", i.part);

    // 生命周期的省略
    // 前提:
    //  - 每个引用都有生命周期
    //  - 需要为使用生命周期的函数或 struct 指定生命周期参数
    // 在 Rust 引用分析中所编入的模式称为: 生命周期省略规则
    //  - 这些规则无需开发者遵守
    //  - 它们是一些特殊情况，由编译器来考虑
    //  - 如果你的代码复合这些情况，那么就无需显式得标注生命周期
    // 生命周期规则不会提供完整的推断
    //  - 如果应用规则后，引用的生命周期仍旧模糊不清 -> 编译错误
    //  - 解决办法: 添加生命周期标注，表明引用间的相互关系

    // 输入、输出生命周期
    // 生命周期在:
    //  - 函数 / 方法的参数   : 输入生命周期
    //  - 函数 / 方法的返回值  : 输出生命周期

    // 生命周期省略的三个规则
    // 编译器使用 3 个规则在没有显式标注生命周期的情况下，来确定引用的生命周期
    //  - 规则 1 应用于输入生命周期
    //  - 规则 2、3 应用于输出生命周期
    //  - 如果编译器应用完三个规则后，仍然无法确定引用的生命周期 -> 报错
    //  - 这些规则适用于 fn 定义和 impl 块

    // 规则 1: 每个引用的类型参数都有自己的生命周期
    // 规则 2: 如果只有一个输入生命周期参数，那么该生命周期会被赋给所有的输出生命周期
    // 规则 3: 如果有多个输入生命周期参数，但其中一个是 &self 或是 &mut self ( 方法 )，
    //         那么 self 的生命周期会被赋给所有的输出生命周期参数

    // 例子:
    trait LifeCycle {
        // 第一个例子
        fn first_world(s: &str) -> &str;
        // 应用第 1 条规则
        fn first_world_rule_1<'a>(s: &'a str) -> &str;
        // 应用第 2 条规则
        fn first_world_rule_2<'a>(s: &'a str) -> &'a str;

        // 第二个例子
        fn longest(x: &str, y: &str) -> &str;
        // 应用第 1 条规则，因为有 2 个输入生命周期且没有 self，所以应用完 3 条规则之后仍旧
        // 无法计算出签名中所有引用的生命周期，所以 Rust 会报错: error[E0106]: missing lifetime specifier
        // fn longest_rule_1<'a, 'b>(x: &'a str, y: &'b str) -> &str;
    }
}