fn first_world(x: &str) -> &str {
    let _bytes = x.as_bytes();
    for (i, &item) in _bytes.iter().enumerate() {
        // b' ' 为 u8 类型，且
        if item == b' ' {
            return &x[..i];
        }
    }
    return &x[..];
}

pub fn learning_slice() {
    println!("Start to learn slices");

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
    // 切片本身是否是一种引用 ? 改变引用会不会改变原先的值 ? ( mut 的类型 )
    // 答案: 是的，属于一种引用，会改变原先的值
    // todo: 下面 2 句话: 173 和 174 对调就会产生 error，详细了解为什么
    println!("slice index 0 value: {}", array_slice[0]);
    println!("origin index 0 value: {}", origin_array[0]);

    // 字符串字面值的类型就是字符串切片! 所以不需要再 &
    let whole = "Hello World";
    let find = first_world(whole);
    println!("first world is: {}", find);
    let my_whole = String::from("My name is jack");
    let find = first_world(&my_whole[..]);
    println!("first world is: {}", find);

    // String 传递给 &str 会被自动转成 &str 类型么？ 答案是不会
    // let find = first_world(my_whole);
}