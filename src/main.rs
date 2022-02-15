use std::fmt::Result;
use std::io::Result as IoResult;

// use 的简写形式
use std::io::{self, Write};


// todo: 需要研究下 项目名:: 这种用法
use rust_course::eat_at_restaurant;

mod front_of_house;

// 引入包内所有的内容
// use std::collections::*;

// mod tree
mod variables;
mod functions;
mod control_flow;
mod ownership;
mod reference;
mod slices;
mod structs;
mod enums;
mod match_syntax;
mod hashmap;
mod string;
mod vector;
mod panic;
mod result;
mod eliminate_duplicate_code;
mod generics;
mod learning_trait;
mod lifecycle_1;
mod lifecycle_2;
mod lifecycle_3;
mod lifecycle_4;
mod learning_test;
mod example_grep;
mod closure_1;
mod closure_2;
mod iterator;

fn main() {
    // Variables and Mutability
    variables::learning_variables();

    // Functions
    functions::learning_functions();

    // Control Flow
    control_flow::learning_control_flow();

    // Ownership
    ownership::learning_ownership();

    // Reference
    reference::learning_reference();

    // Slices
    slices::learning_slice();

    // Struct
    structs::learning_struct();

    // Enum & Pattern matching
    enums::learning_enums();

    // match 关键字
    // 允许一个值与一系列模式进行匹配, 模式
    match_syntax::learning_match();

    // 使用 module 下的函数
    eat_at_restaurant();
    front_of_house::hosting::print_house();
    front_of_house::hosting::print_house();

    // vector
    vector::learning_vector();

    // string
    string::learning_string();

    // hashmap 章节
    hashmap::learning_hashmap();

    // panic
    panic::learning_panic();

    // result
    result::learning_result();

    // eliminate duplicated code
    eliminate_duplicate_code::learning_eliminate();

    // generics
    generics::learning_generics();

    // trait
    learning_trait::learning_trait();

    // lifecycle
    lifecycle_1::learning_lifecycle();
    lifecycle_2::learning_lifecycle();
    lifecycle_3::learning_lifecycle();
    lifecycle_4::learning_lifecycle();

    // test
    learning_test::learning_tests();

    // grep example
    example_grep::learning_grep_example();
    example_grep::my_grep();

    // closure
    closure_1::learning_closure();
    closure_2::learning_closure();

    // iterator
    iterator::learning_iterator();
}
