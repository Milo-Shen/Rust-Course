use std::thread;
use std::time::Duration;

pub fn learning_closure() {
    println!("Start to learn closure 1");

    // 什么是闭包 ( closure )
    // 闭包: 可以捕获其所在环境的匿名函数
    //  - 是匿名函数
    //  - 保存为变量、作为参数
    //  - 可以在一个地方创建闭包，然后在另一个上下文中调用闭包来完成运算
    //  - 可以从其定义的作用域捕获值
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly... ");
    thread::sleep(Duration::from_secs(1));
    return intensity;
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 此处是把匿名函数的定义传给了变量 expensive_closure
    // 此时 expensive_closure 的函数签名为: fn(u32) -> u32
    let expensive_closure: fn(u32) -> u32 = |num: u32| {
        println!("calculating slowly... ");
        thread::sleep(Duration::from_secs(1));
        return num;
    };
    if intensity < 25 {
        println!("Today, do {} push ups!", expensive_closure(intensity));
        println!("Next, do {} sit ups", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}