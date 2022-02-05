pub fn learning_generics() {
    println!("Start to learn generics");

    // generic function
    fn my_value<T>(x: T) -> T {
        return x;
    }

    let return_value: &str = my_value("Hello World");
    println!("The return value of generic function is: {}", return_value);

    // generic struct
    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let mut point: Point<i32, f64> = Point { x: 1, y: 2.0 };
    println!("The value of generic struct is: {:#?}", point);

    // generic enums
    // 可以让枚举的变体拥有泛型数据类型，例如: Option<T> 和 Result<T, E>
    enum _Option<T> {
        Some(T),
        None,
    }

    enum _Result<T, E> {
        Ok(T),
        Err(E),
    }

    // generic method
    // 把 T 放在 impl 关键字后，标识在类型 T 上实现方法，例如 impl<T> Point<T>
    // 只针对具体类型实现方法 ( 其余类型没实现方法 )，例如 impl Point<f32>
    impl<T, U> Point<T, U> {
        fn reference(&mut self) -> &mut Point<T, U> {
            // 此处的 self 是引用类型 ( rust 里称为借用 )
            return self;
        }

        fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point { x: self.x, y: other.y }
        }
    }

    impl Point<i32, i32> {
        fn specific(&self) {
            println!("This is a special function for Point<i32, i32>");
        }
    }

    let mut another_point = point.reference();
    another_point.x = 100;
    another_point.y = 200.0;
    println!("The origin value x: {}, y: {}", point.x, point.y);

    let i32_point = Point { x: 1, y: 1 };
    // the below function is specific to Point<i32, i32>
    i32_point.specific();

    // struct 里的泛型参数可以和方法的泛型类型参数不同
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: "Hello", y: "world" };
    let p3 = p1.mix_up(p2);
    println!("The mixed up value p3 is: {:#?}", p3);

    // 使用泛型的代码和使用具体类型代码的运行速度是一样的
    // 单态化-在编译时将泛型类型替换为具体类型的过程
}