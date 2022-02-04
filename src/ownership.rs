fn take_ownership(x: String) {
    println!("take ownership: {}", x);
}

fn calculate_length(x: String) -> (String, usize) {
    let length = x.len();
    return (x, length);
}

pub fn learning_ownership() {
    println!("Start to learn ownership");

    let mut name = String::from("jack");
    name.push_str("hello");
    println!("my name is: {}", name);

    let another_str = name.clone();
    println!("my name is: {}", name);

    let ownership_str = String::from("jack");
    take_ownership(ownership_str);
    // print!("{}", ownership_str); value borrowed here after move

    let ownership_str = String::from("jack");
    let (ownership_str_1, length) = calculate_length(ownership_str);
    println!("size of String {} is: {}", ownership_str_1, length);
}