const VERSION: &str = "1.0.0";

pub fn learning_variables() {
    // Variables and Mutability
    println!("Start to learn variables and mutability");
    // a address is :0x140701963303620
    let a: i32 = 5;
    let address = & a as * const i32 as usize;
    println!("a address is :0x{}", address);

    // a address is :0x140701963303708
    let a: i32 = 10;
    let address = & a as * const i32 as usize;
    println!("a address is :0x{}", address);

    // b address is :0x140701963303796
    let mut b: i32 = 5;
    let address = & b as * const i32 as usize;
    println!("b address is :0x{}", address);

    // b address is :0x140701963303796
    // rust don't update the address of variable b if we use mut syntax
    b = 50;
    let address = & b as * const i32 as usize;
    println!("b address is :0x{}", address);
    let tuple: (i32, f64) = (1, 2.0);
    println!("my tuple is: {}, {}", tuple.0, tuple.1);
    let (_a, _b) = tuple;
    println!("my tuple is: {}, {}", _a, _b);
    let array_a: [i32; 2] = [1, 2];
    let array_b = [3; 2];
    let mut array_c = [1, 2];
    println!("array value: {}, {}", array_a[0], array_b[0]);
    println!("array len of array_b is: {}", array_b.len());
}