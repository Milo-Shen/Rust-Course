use rust_course::mix;
use rust_course::PrimaryColor;
use rust_course::SecondaryColor;

pub fn learning_pub_use() {
    println!("Start to learn pub use");

    // 使用 pub use 导出方便使用的公共 API
    // 问题: crate 的程序结构在开发时对于开发者很合理，但对于它的开发者不够方便
    //  - 开发者会把程序分为很多层，使用者想找到这种深层结构中的某个类型很费劲

    // 例如:
    //  - 麻烦: my_crate::some_module::another_module::UsefulType;
    //  - 方便: my_create::UsefulType

    // 解决办法:
    //  - 不需要重新组织内部代码结构
    //  - 使用 pub use: 可以重新导出，创建一个与内部私有结构不同的对外公共结构

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let color: SecondaryColor = mix(red, yellow);
    println!("The mixed color is: {:?}", color)
}
