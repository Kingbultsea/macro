mod abc;
mod impl_l;

mod trait_l;

mod dir_2 {
    pub mod pub_mod;

    pub mod child;
}

mod dir_1 {
    pub mod my_app;
    pub mod dir_1_file;
    pub mod dir_2_file;
}

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

    trait_l::borrow_trait::run_b();

    dir_2::pub_mod::init();
}
