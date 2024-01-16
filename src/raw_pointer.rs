pub fn learning_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("{:?}", r1);
    println!("{:?}", r2);

    let address = 0x012345usize;
    let r = address as *const i32;

    // 只有在 unsafe 中才能解引用裸指针
    unsafe {
        println!("{:?}", *r1);
        println!("{:?}", *r2);
    }
}
