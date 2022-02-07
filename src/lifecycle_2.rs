pub fn learning_lifecycle() {
    println!("Start to learn life cycle - 2");

    // 生命周期的标注不会改变引用的生命周期的长度
    // 当指定了泛型生命周期参数，函数可以接收带有任意生命周期的引用
    // 生命周期的标注：描述了多个引用的生命周期间的关系，但不影响生命周期

    // 生命周期标注的语法
    // 生命周期参数名：
    //   - 以 ' 开头
    //   - 通常全小写且非常简短
    //   - 通常使用 'a
    // 生命周期标注的位置
    //   - 在引用的 & 符号后
    //   - 使用空格将标注和引用类型分开
    // 泛型生命周期参数生命在：函数名和参数列表之间的 <> 里面

    // 此处泛型定义内 'a 的生命周期为 x, y 生命周期的交集，也就是生命周期较短的那个变量的生命周期
    fn longest_1<'a>(x: &'a str, y: &'a str) -> &'a str {
        return if x.len() > y.len() {
            x
        } else {
            y
        };
    }

    let string_1 = String::from("Hello World");
    // Rust 里没有 NULL
    let result: &str;
    {
        // &str 类型的字符串是 static 生命周期，在整个程序执行期间都有效
        // 所以这段代码能编译通过
        let string_2 = "Jack";
        result = longest_1(string_1.as_str(), string_2);
    }
    println!("The longest string is: {}", result);

    // 生命周期 'a 的实际生命周期是：x 和 y 两个生命周期中较小的那一个
    let string_1 = String::from("Hello World");
    // Rust 里没有 NULL
    let mut result: &str = "string_2: borrowed value does not live long enough";
    {
        // &str 类型的字符串是 static 生命周期，在整个程序执行期间都有效
        // 所以这段代码能编译通过
        let string_2 = String::from("Jack");
        // string_2: borrowed value does not live long enough
        // 此处的 string_2 没有足够的生命周期，其生命周期到 49 行就结束了
        // result = longest_1(string_1.as_str(), string_2.as_str());
    }
    // result 的生命周期为 string_1 和 string_2 中较小的那个，然而
    // string_2 的生命周期在 49 行就结束了，所以下面的代码会报错
    println!("The longest string is: {}", result);
}