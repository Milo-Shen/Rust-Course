pub fn learning_unsafe_rust() {
    println!("start to learn unsafe rust");

    // 不安全 Rust
    // 匹配命名变量
    // 隐藏着第二个语言, 它没有强制内存安全保证: unsafe rust ( 不安全的 rust )
    //  - 和普通 rust 一样, 但提供了额外的超能力

    // unsafe rust 存在的原因
    //  - 静态分析是保守的
    //    - 使用 unsafe rust: 我知道自己在做什么, 并承担相应的风险
    //  计算机硬件本身就是不安全的, rust 需要能够进行底层系统编程

    // unsafe 超能力
    // 使用 unsafe 关键字切换到 unsafe rust, 开启一个块, 里面放着 unsafe 代码
    // unsafe rust 里可执行的四个动作 ( unsafe 超能力 )
    //  - 解引用原始指针
    //  - 调用 unsafe 函数或方法
    //  - 访问或修改可变的静态变量
    //  - 实现 unsafe trait

    // 注意:
    // unsafe 并没有关闭借用检查或停用其他安全检查
    // 任何内存安全的错误必须停留在 unsafe 块里
    // 尽可能隔离 unsafe 代码, 最好将其封装在安全的抽象里, 提供安全的 API
}