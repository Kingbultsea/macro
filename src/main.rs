mod abc;
mod impl_l;

mod dir_2 {
    pub mod pub_mod;
}

mod dir_1;

// 需要定义个crate 知道使用了宏里面的哪些方法
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
    abc::use_abc();

    let person = impl_l::Person {
        name: String::from("liliya"),
        years: 8,
    };

    impl_l::hello_impl();

    dir_1::dir_1_file::call_my_name();

    person.my_name();
    person.my_years();

    dir_2::pub_mod::init();
}
