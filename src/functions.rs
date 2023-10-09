fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn empty_function() {}

fn plus_five(x: i32) -> i32 {
    return x + 5;
}

fn plus_six(x: i32) -> i32 {
    x + 6
}

pub fn learning_functions() {
    println!("Start to learn functions");
    another_function(10086);
    let mut total: i32 = 0;
    total = plus_five(total);
    println!("The total value is: {}", total);
    total = plus_six(total);
    println!("The total value is: {}", total);

    let y = {
        let x = 3;
        // 此处 x + 1 后不可加分号，加分号的话返回值为 Tuple
        x + 1
    };

    println!("The value of y is: {}", y);

    // Empty Functions
    // todo: The return value of empty function is ();
    let empty_result = empty_function();
}
