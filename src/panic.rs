pub fn learning_panic() {
    println!("Start to learn panic");

    // Rust 错误的分类
    // 可恢复的错误: 例如文件未找到，可再次尝试 Result<T,E>
    // 不可恢复的错误：bug                   panic!
    // Rust 没有类似异常的机制
    // panic!("crash and burn");
    let arr = vec![1, 2, 3];
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // todo: mac os 上使用 set RUST_BACKTRACE=1 无效
    // let b = arr[100];

    struct Guess {
        _value: i32,
    }

    impl Guess {
        fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", value);
            }
            return Guess { _value: value };
        }

        fn value(&self) -> i32 {
            return self._value;
        }
    }

    let mut guess_num = Guess::new(50);
    guess_num._value = 100;
    let my_num = guess_num.value();
    println!("The guess value is: {}", my_num);
}