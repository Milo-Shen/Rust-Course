// struct, enum，其他: 指定完整的路径 （ 指定到本身 ）
use std::collections::HashMap;

pub fn learning_hashmap() {
    println!("Start to learn hashmap");

    // 创建 HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("blue"), 10);

    // 使用 collection 创建 HashMap
    let teams: Vec<String> = Vec::from([String::from("Blue"), String::from("Yellow")]);
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    // 对于实现了 copy trait 的类型，值会被复制到 HashMap 中
    // 对于拥有所有权的值 (例如 String)，值会被移动，所有权会转移给 HashMap
    let field_name = String::from("favourite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    // field_name 和 field_value 的所有权被转移到了 map 中
    map.insert(field_name, field_value);
    // 因为此处已经没有 field_name 和 field_value 的所有权了，所以无法访问
    // error[E0382]: borrow of moved value: `field_value`, `field_name`
    // println!("{}, {}", field_name, field_value);

    // 此时的 field name 失去了所有权，error[E0382]: borrow of moved value: `field_name`
    // println!("{},{}", field_name, field_value);

    // 如果将值的引用插入到 HashMap，值本身不会移动
    // 在 HashMap 有效的期间内，被引用的值必须保持有效
    let mut map: HashMap<&String, &String> = HashMap::new();
    let field_name = String::from("favourite color");
    let field_value = String::from("Blue");
    map.insert(&field_name, &field_value);
    println!("{},{}", field_name, field_value);

    // 访问 HashMap 中的值
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    let team_name = String::from("Blue");
    // 此处编译器使用了技术：解引用强制转换，把 &String 的类型转换成了 &str
    let score = scores.get(&team_name);
    // 使用 match 语法获得 HashMap 的 return type: Option
    match score {
        Some(s) => println!("use match syntax: {}", s),
        None => println!("use match team not exist")
    }
    // 使用 if let 来获得值
    if let Some(s) = score {
        println!("use if let syntax: {}", s);
    }

    // 遍历 HashMap
    // todo: 此处是否会调用迭代器 ?
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // 更新 HashMap - 替换原有的值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:#?}", scores);

    // 更新 HashMap - 不存在待插入的值的 key 时才插入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    let inserted_yellow = scores.entry(String::from("Yellow")).or_insert(50);
    let inserted_blue = scores.entry(String::from("Blue")).or_insert(50);
    println!("{:#?}", scores);

    // 更新 HashMap - 基于现有的 value 来更新 value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);

    // 往 map 里插入引用的做法 ( 需要配合生命周期来确保引用的有效性 )
    let mut map: HashMap<i32, &String> = HashMap::new();
    {
        let str = String::from("temporary value");
        // error[E0597]: `str` does not live long enough
        // 此处 str 的生命周期短于 map 的生命周期, 所以下面的代码会报错
        // map.insert(1, &str);
    }
    println!("{:?}", map);
}