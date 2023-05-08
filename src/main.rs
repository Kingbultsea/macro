mod cons;
mod rc;
mod iterator;
mod closure;
mod abc;
mod impl_l;
mod run_rocket;

mod trait_l;

mod life_time;

mod dir_2 {
    pub mod pub_mod;

    pub mod child;
}

mod dir_1 {
    pub mod my_app;
    pub mod dir_1_file;
    pub mod dir_2_file;
}

mod default;

// 需要定义个crate 知道使用了宏里面的哪些方法
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use trait_l::borrow_trait::People;

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

    let pepole = trait_l::borrow_trait::run_b();
    pepole.my_sex();
    trait_l::pair::run_pair();
    trait_l::latest::run_latest();
    trait_l::outline_print::run_outline_print();
    trait_l::wrapper::run_wrapper();

    dir_2::pub_mod::init();
    trait_l::trait_struct::run_trait_struct();

    let mut abc = String::from("aasdas");

    let borrow_mut_abc = &mut abc;

    borrow_mut_abc.push_str("asd");
    abc.push_str("abcdef");
    
    life_time::life::run_life();

    default::run_default::run_default();
    default::re_borrow::re_borrow();

    closure::main::run_closure();
    closure::cache::run_cache();
    closure::fn_mut::run_fn_mut();
    iterator::run::run_iteraotr();

    cons::run_cons::run_rs();
    rc::run_rc::main();
}
