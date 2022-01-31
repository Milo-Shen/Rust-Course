fn serve_order() {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {
            println!("add to wait list successfully");
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

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            return Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            };
        }
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_wait_list();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}