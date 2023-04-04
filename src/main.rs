use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
pub struct Pancakes;

#[derive(HelloMacro)]
pub struct Waffles;

fn main() {
    Pancakes::hello_macro();
    println!("Hello, world!");
}
