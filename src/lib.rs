fn serve_order() {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {
            println!("add to wait list successfully");
        }

        fn private_function() {
            println!("This is a private function");
        }
    }

    fn fix_order() {
        super::serve_order();
        crate::serve_order();
    }
}

mod back_of_house {
    // pub 放在 struct 前, struct 是公共的，而 struct 的字段默认是私有的
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // pub enum
    // 公共枚举里面的所有变体都是公共的
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            };
        }
    }
}

use crate::front_of_house::hosting;

// 下面是使用相对路径引入
// use front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_wait_list();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // use absolute path: hosting
    hosting::add_to_wait_list();
    // Function `private_function` is private [E0603] use 也需要遵守私有性规则
    // hosting::private_function();

    // todo: use 的习惯用法，函数: 将函数的父级模块引入到作用域
    // todo: struct, enum，其他: 指定完整的路径 （ 指定到本身 ）
    // todo: 但是对于同名条目：指定到父级
}

