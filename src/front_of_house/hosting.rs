// 目录的结构，必须和单元包的结构相同
pub fn add_to_wait_list() {
    println!("add to wait list successfully");
}

fn private_function() {
    println!("This is a private function");
}

pub fn print_house() {
    println!("This is front_of_house_hosting module");
    private_function();
}

pub mod test_mod {
    use super::*;

    pub fn hello() {
        println!("hello test mod - private function");
        private_function();
    }
}