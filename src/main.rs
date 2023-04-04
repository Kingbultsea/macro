mod abc;
use crate::abc::use_abc;
// 需要定义个crate 知道使用了宏里面的哪些方法
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}