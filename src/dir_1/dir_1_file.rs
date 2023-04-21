mod dir_2_file {
    pub fn my_fn() {
        println!("my fn");
    }
}

mod my_app {
    pub fn use_my_app_1() {
         println!("1");
     }
 }

mod hello_macro {
    pub use crate::impl_l::hello_impl;

    pub fn foo() {
        println!("HELLO");
    }
}

pub fn call_my_name() {
    hello_macro::hello_impl();
    hello_macro::foo();

    dir_2_file::my_fn();

    my_app::use_my_app_1();

    println!("hello 22");
}
