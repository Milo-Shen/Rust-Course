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

fn take_ownership(x: String) {
    println!("take ownership: {}", x);
}

fn calculate_length(x: String) -> (String, usize) {
    let length = x.len();
    return (x, length);
}

fn calculate_length_1(x: &String) -> usize {
    // x.push_str("world"); Cannot borrow immutable local variable `x` as mutable
    x.len()
}

fn mut_calculate_length(x: &mut String) -> usize {
    x.push_str("_mut");
    x.len()
}

fn first_world(x: &str) -> &str {
    // todo: &[u8] 是什么 ?
    let _bytes = x.as_bytes();
    for (i, &item) in _bytes.iter().enumerate() {
        if item == b' ' {
            return &x[..i];
        }
    }
    return &x[..];
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
    println!("array len of array_b is: {}", array_b.len());


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


    // Ownership
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


    // Reference
    let str = String::from("Hello");
    let length = calculate_length_1(&str);
    println!("my str: {}'s length is: {}", str, length);

    // mut reference
    let mut mut_str: String = String::from("Hello");
    let length = mut_calculate_length(&mut mut_str);
    println!("Len of updated mutable variable {} is: {}", mut_str, length);

    let mut mut_str_another = String::from("Hello");
    let s1 = &mut mut_str_another;
    // let s2 = &mut mut_str_another; cannot borrow `mut_str_another` as mutable more than once at a time
    // println!("{}{}", s1, s2);

    let a: char = 'a';
    let b: char = 'a';
    let c: bool = a == b;
    println!("{}", c);

    // string slice
    let str_slice = String::from("Hello World");
    let hello: &str = &str_slice[..5];
    let world: &str = &str_slice[6..];
    let whole: &str = &str_slice[..];
    println!("{},{},{}", hello, world, whole);

    // array slice
    let origin_array = [1; 4];
    let array_slice = &origin_array[..2];
    println!("array_slice index 1 is: {}", array_slice[1]);

    // mut array slice
    let mut origin_array = [1; 4];
    let array_slice = &mut origin_array[..2];
    array_slice[0] = 10;
    // todo: 切片本身是否是一种引用 ? 改变引用会不会改变原先的值 ? ( mut 的类型 )
    // todo: 下面 2 句话: 173 和 174 对调就会产生 error，详细了解为什么
    println!("slice index 0 value: {}", array_slice[0]);
    println!("origin index 0 value: {}", origin_array[0]);

    // todo: 字符串字面值的类型就是字符串切片 ! 所以不需要再 &
    let whole = "Hello World";
    let find = first_world(whole);
    println!("first world is: {}", find);
    let my_whole = String::from("My name is jack");
    let find = first_world(&my_whole[..]);
    println!("first world is: {}", find);


    // struct
    struct User {
        name: String,
        age: u8,
    }
    let jack = User {
        age: 1,
        name: String::from("jack"),
    };
    let marry = User {
        name: String::from("Mary"),
        ..jack
    };
    println!("name: {}, {}", jack.name, marry.name);

    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(f32, f32, f64);
    let black: Color = Color(0, 0, 0);
    let point: Point = Point(1.0, 1.0, 1.0);
    println!("{}, {}", black.0, point.0)
}
