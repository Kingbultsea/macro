mod abc;
mod impl_l;

// 需要定义个crate 知道使用了宏里面的哪些方法
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

enum Lang {
    English,
    Russian,
}

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
    abc::use_abc();

    let person = impl_l::Person {
        name: String::from("liliya"),
        years: 8,
    };

    person.my_name();
    person.my_years();
}
