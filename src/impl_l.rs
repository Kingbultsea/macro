pub struct Person {
    // 可变的用String 不可变的用&str
    pub(crate) name: String,
    pub(crate) years: u8,
}

impl Person {
    // &self 其实是 self: &Self
    pub fn my_name(&self) {
        println!("{}", self.name);
    }

    pub fn my_years(&self) {
        print!("{}", self.years);
    }
}

pub fn hello_impl() {
    println!("123");
}
