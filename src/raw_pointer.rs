pub fn learning_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    println!("{:?}", r1);
}
