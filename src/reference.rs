fn calculate_length_1(x: &String) -> usize {
    // x.push_str("world"); Cannot borrow immutable local variable `x` as mutable
    x.len()
}

fn mut_calculate_length(x: &mut String) -> usize {
    x.push_str("_mut");
    x.len()
}

pub fn learning_reference() {
    println!("Start to learn reference");

    let str = String::from("Hello");
    let length = calculate_length_1(&str);
    println!("my str: {}'s length is: {}", str, length);

    // mut reference
    let mut mut_str: String = String::from("Hello");
    let length = mut_calculate_length(&mut mut_str);
    println!("Len of updated mutable variable {} is: {}", mut_str, length);

    let mut mut_str_another = String::from("Hello");
    let s1 = &mut mut_str_another;
    s1.push_str("_modified");
    println!("mut_str_another: {}", mut_str_another);
    // let s2 = &mut mut_str_another; cannot borrow `mut_str_another` as mutable more than once at a time
    // println!("{}{}", s1, s2);

    let a: char = 'a';
    let b: char = 'a';
    let c: bool = a == b;
    println!("Is a equal to b ?: {} ", c);
}
