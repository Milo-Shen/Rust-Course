pub fn learning_control_flow() {
    println!("Start to learn control flow");

    let number: i32 = 3;

    // condition 必须是布尔类型
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if 是一个表达式
    let final_number = if number < 5 { 1 } else { 2 };
    println!("final number is: {}", final_number);

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    for numbers in 1..5 {
        println!("i32 value is: {}", numbers);
    }

    for numbers in (1..5).rev() {
        println!("i32 value is: {}", numbers);
    }

    let print_array: [i32; 4] = [1, 2, 3, 4];

    for i in print_array {
        println!("i32 value is: {}", i);
    }

    for i in print_array.iter() {
        println!("&i32 value is: {}", i);
    }

    for numbers in 1..=5 {
        println!("i32 value is: {}", numbers);
    }
}
