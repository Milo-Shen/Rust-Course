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

    let x = 32;
    fn f() {
        // 编译错误，不能访问函数外面的变量x和y
        // println!("{}", x);
    }

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

    // 在Rust中，能否访问外部变量称为【捕获环境】。比如函数是不能捕获环境的，而大括号可以捕获环境
    let mut a = 33;
    {
        // 可以访问大括号外面的变量a
        a += 1;
    }
    println!("{}", a);

    // 对于可捕获环境的大括号作用域，要注意Rust的变量遮盖行为
    let mut a = 33;
    {
      a += 1;   // 访问并修改的是外部变量a的值
  
      // 又声明变量a，这会发生变量遮盖现象
      // 从此开始，大括号内访问的变量a都是该变量
      let mut a = 44; 
      a += 2;
      println!("{}", a);  // 输出46
    }    // 大括号内声明的变量a失效
    println!("{}", a);   // 输出34
}
