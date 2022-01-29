const VERSION: &str = "1.0.0";

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn plus_five(x: i32) -> i32 {
    return x + 5;
}

fn plus_six(x: i32) -> i32 {
    x + 6
}

fn main() {
    // Variables and Mutability
    println!("rust course version is: {}", VERSION);
    let a: i32 = 5;
    let address = &a as *const i32 as usize;
    println!("a address is :0x{}", address);
    let a: i32 = 10;
    let address = &a as *const i32 as usize;
    println!("a address is :0x{}", address);
    let mut b: i32 = 5;
    let address = &b as *const i32 as usize;
    println!("b address is :0x{}", address);
    b = 50;
    let address = &b as *const i32 as usize;
    println!("b address is :0x{}", address);
    let tuple: (i32, f64) = (1, 2.0);
    println!("my tuple is: {}, {}", tuple.0, tuple.1);
    let (_a, _b) = tuple;
    println!("my tuple is: {}, {}", _a, _b);
    let array_a: [i32; 2] = [1, 2];
    let array_b = [3; 2];
    println!("array value: {}, {}", array_a[0], array_b[0]);


    // Functions
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


    // Control Flow
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
}
