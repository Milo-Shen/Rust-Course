pub fn learning_string() {
    println!("Start to learn string");

    // create a string (utf-8)
    let mut s = String::new();
    s.push_str("Hello String");
    let mut s = "Hello String".to_string();
    let mut s = String::from("Hello String");

    // update a string
    let mut s = String::from("foo");
    let _append = String::from("_bar");
    // push_str 不会获得所有权
    s.push_str("_bar");
    println!("{}, {}", s, _append);
    let _append_char = 'h';
    // push 方法也不会获得所有权
    s.push(_append_char);
    println!("{}, {}", s, _append_char);

    // syntax +
    // + 号使用了类似这个签名的方法 fn add(self, s: &str) -> String {...}
    let s1 = String::from("Hello ");
    let s2 = String::from("World");
    // 此处的 &s2 使用了 "解引用强制转换 ( deref coercion )"
    let s3 = s1 + &s2;
    // error[E0382]: borrow of moved value: `s1`
    // s1 被借用了，所以此处不能访问 s1，s2 是引用，所以仍旧拥有所有权，此处可以访问
    println!("{},{}", s3, s2);

    // use format! 拼接字符串不会获得所有权
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3: String = String::from("Jack");
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{},{},{},{}", s4, s1, s2, s3);

    // String 类型无法使用索引的形式访问
    let s = String::from("Hello World");
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    // rust 的字符串不支持下标索引语法访问
    // rust 不允许索引访问字符串是因为索引理应消耗 O(1) 时间，但是 String 需要遍历所有内容需要 O(n)
    // let h = s[0];

    // String 类型的内部表示 Vec<v8>
    let len = String::from("Hello World").len();
    println!("The length of string is: {}", len);

    // Bytes, Scalar Values, Grapheme Clusters
    // 字节，标量值，字形簇（最接近所谓的"字母"）
    let s = "My name is jack";
    // 按字节访问字符串
    for byte in s.bytes() {
        println!("{}", byte);
    }
    for ch in s.chars() {
        println!("{}", ch);
    }

    // 切割 String
    let s = "我的名字叫 Jack";
    let s1 = &s[..3];
    println!("The slice of s is: {}", s1);
    // thread 'main' panicked at 'byte index 2 is not a char boundary; it is inside
    // 切割字符串时，一定要按照字符的边界进行切割，否则会引起恐慌 panic
    // let s2 = &s[..2];

    // as_str 函数
    let mut string_1: String = " I am a string ".to_string();
    let mut my_str: &str = string_1.as_str();
    my_str = my_str.trim();
    // The value of my str is:| I am a string |,|I am a string|
    println!("The value of my str is:|{}|,|{}|", string_1, my_str);
}