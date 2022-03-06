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
mod iterator_1;
mod iterator_2;
mod iterator_3;
mod iterator_4;
mod iterator_5;
mod iterator_6;
mod learning_pub_use;
mod smart_pointer_begin;
mod smart_pointer_box;
mod deref_trait_1;
mod deref_trait_2;
mod drop_trait;
mod rc;
mod refcell;
mod memory_leak;
mod threads_1;
mod threads_2;
mod threads_3;
mod threads_4;
mod oop_1;
mod oop_2;
mod oop_3;
mod pattern_matching;

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
    iterator_1::learning_iterator();
    iterator_2::learning_iterator();
    iterator_3::learning_iterator();
    iterator_4::learning_iterator();
    iterator_5::learning_iterator();
    iterator_6::learning_iterator();

    // pub use
    learning_pub_use::learning_pub_use();

    // smart pointer
    smart_pointer_begin::learning_smart_pointer();
    smart_pointer_box::learning_smart_pointer();

    // deref trait
    deref_trait_1::learning_deref_trait();
    deref_trait_2::learning_deref_trait();

    // drop trait
    drop_trait::learning_drop_trait();

    // Rc<T> 引用计数智能指针
    rc::learning_rc();

    // RecCell<T>
    refcell::learning_ref_cell();

    // memory leak
    memory_leak::learning_memory_leak();

    // multi threads
    threads_1::learning_threads();
    threads_2::learning_threads();
    threads_3::learning_threads();
    threads_4::learning_threads();

    // oop
    oop_1::learning_oop();
    oop_2::learning_oop();
    oop_3::learning_oop();

    // pattern matching
    pattern_matching::learning_pattern_matching();
}
