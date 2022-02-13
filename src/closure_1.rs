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

    // 闭包的类型推断
    //  - 注意: 闭包的定义最终只会为参数 / 返回值推断出唯一具体的类型
    // todo: 为什么加个 return 就不提示: error[E0282]: type annotations needed
    // 不标注类型，且没有被调用时，无法通过编译，会提示添加类型申明
    let example_closure = |x| x;
    // 调用下方函数后，x 的类型被确定为了 &str
    let s = example_closure("Hello Closure");
    // 因为上面语句的执行中 x 已经是 &str 了，所以再赋 int 类型是错误的，
    // 错误提示为: error[E0308]: mismatched types, expected `&str`, found integer
    // let n = example_closure(5);
}

struct Cache<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cache<T>
    where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cache<T> {
        return Cache {
            calculation,
            value: None,
        };
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                return v;
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 此处是把匿名函数的定义传给了变量 expensive_closure
    // 此时 expensive_closure 的函数签名为: fn(u32) -> u32
    let mut expensive_closure: Cache<fn(u32) -> u32> = Cache::new(|num: u32| -> u32 {
        println!("calculating slowly... ");
        thread::sleep(Duration::from_secs(1));
        return num;
    });
    if intensity < 25 {
        // 此处的 expensive_closure 会被执行 2 次，比较浪费性能，下面给出优化的手段
        // 创建一个 struct，它持有闭包及其调用的结果:
        //  - 只会在需要结果时才执行该闭包
        //  - 可缓存结果
        // 这个模式通常叫做记忆化 ( memoization ) 或延迟计算 ( lazy evaluation )

        // 如何让 struct 持有闭包
        // struct 的定义需要知道所有字段的类型
        //  - 需要知名闭包的类型
        // 每个闭包实例都有自己唯一的匿名类型,即使两个闭包签名完全一致
        // 所以需要使用: 泛型和 Trait Bound

        // Fn Trait ( 由标准库提供 )
        // 所有的闭包都实现了至少以下 Trait 之一 :
        //  - Fn
        //  - FnMut
        //  - FnOnce
        println!("Today, do {} push ups!", expensive_closure.value(intensity));
        println!("Next, do {} sit ups", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

// 闭包的类型推断和类型标注
// 闭包的类型推断
//  - 闭包不要求标注参数和返回值的类型，但是函数和方法需要
//  - 闭包通常很短小，只在狭小的上下文中工作，编译器通常能推断出类型
//  - 可以手动添加类型标注

// 函数和闭包定义的语法
// fn add_one_v1   ( x: u32 ) -> u32 { x + 1 }
// fn add_one_v2 = | x: u32 | -> u32 { x + 1 };
// fn add_one_v3   |x|               { x + 1 };
// fn add_one_v4   |x|                 x + 1  ;

// 闭包的类型推断
//  - 注意: 闭包的定义最终只会为参数 / 返回值推断出唯一具体的类型

// 使用泛型参数和 Fn Trait 来存储闭包