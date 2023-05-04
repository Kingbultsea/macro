use std::fmt::Display;

// 裹一层防止孤儿规则（即类型 和 trait至少有一个是在当前定义的）
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn run_wrapper() {
    let a = Wrapper(vec![String::from("abc"), String::from("efg")]);
    println!("a = {}", a);
}