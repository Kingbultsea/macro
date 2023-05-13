// 裸指针没有实现send，为裸指针实现send
use std::thread;

#[derive(Debug)]

struct MyBox(*mut u8);
unsafe impl Send for MyBox {}

pub fn main() {
    let p = MyBox(5 as *mut u8);

    let t = thread::spawn(move || {
        println!("{:?}", p);
    });

    t.join().unwrap();
}