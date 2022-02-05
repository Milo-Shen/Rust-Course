pub fn learning_eliminate() {
    println!("Start to learn eliminate duplicated code");

    // 此处传入的 list 是 i32 数组的切片，注意此处的参数是切片
    // 切片实际上就是一个引用
    fn largest(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for item in list {
            if *item > largest {
                largest = *item;
            }
        }
        return largest;
    }

    let vector = vec![1, 2, 3];
    let largest = largest(&vector);
    println!("The largest value is: {}", largest);
}