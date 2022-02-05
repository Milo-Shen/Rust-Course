pub fn learning_vector() {
    println!("Start to learn vector");

    // Create a vector
    let v: Vec<i32> = Vec::new();

    // Use initial values to create a vector
    let v = vec![1, 2, 3];

    // use mut vector
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    println!("The first element is: {}", v[0]);

    // use reference to visit vector
    let second: &i32 = &v[1];
    println!("The second element is: {}", second);

    // use get method & match syntax to get value
    match v.get(0) {
        Some(first_value) => println!("Match -> the first value is: {}", first_value),
        None => println!("The vector is empty")
    }

    // out of bound process
    // thread 'main' panicked at 'index out of bounds: the len is 2 but the index is 100'
    // let out_of_bound: &i32 = &v[100];    panic

    // code will run into None condition
    match v.get(100) {
        Some(first_value) => println!("Match -> the first value is: {}", first_value),
        None => println!("Out of bound")
    }

    // traverse the vector - immutable
    let v = vec![100, 32, 57];
    // 此处若是使用 v 而不是 &v 的话，这里的 in 后面的 v 就会获得所有权，在循环之后的代码将无法使用 v
    for i in &v {
        println!("the value is: {}", i);
    }

    // traverse the vector - mutable
    let mut v: Vec<i32> = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("the modified value is: {}", i);
    }

    // use enum to store several types of data
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(1),
        SpreadSheetCell::Float(2.0),
        SpreadSheetCell::Text("Text SpreadSheetCell".to_string()),
    ];

    let first_enum_element = &row[0];
    if let SpreadSheetCell::Int(value) = first_enum_element {
        println!("if let match SpreadSheetCell::Int, value is: {}", value);
    }
}