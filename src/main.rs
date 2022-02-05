use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

// use 的简写形式
use std::io::{self, Write};

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
}
