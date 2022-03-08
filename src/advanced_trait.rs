pub fn learning_advanced_trait() {
    println!("Start to learn unsafe trait");

    struct Count {}

    // 在 trait 定义中使用关联类型来指定占位类型
    // 关联类型 ( associated type ) 是 Trait 中的类型占位符, 它可以用于 trait 的方法签名中:
    //  - 可以定义出包含某些类型的 trait, 而在实现前无需知道这些类型是什么
    pub trait MyIterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    impl MyIterator for Count {
        // 在具体实现 trait 的时候再指定类型
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    // 关联类型和泛型的区别
    //       泛型                                   关联类型
    // 每次实现 Trait 时标注类型                     无需标注类型 ( 但是要在里面指明具体的关联类型 )
    // 可以为一个类型多次实现某个 Trait                无法为单个类型多次实现某个 trait
    // ( 不同的泛型参数 )

    pub trait MyIterator2<T> {
        fn next(&mut self) -> Option<T>;
    }

    impl MyIterator2<String> for Count {
        fn next(&mut self) -> Option<String> {
            None
        }
    }

    impl MyIterator2<u32> for Count {
        fn next(&mut self) -> Option<u32> {
            None
        }
    }
}