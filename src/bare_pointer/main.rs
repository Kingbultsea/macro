// 裸指针没有实现send，为裸指针实现send
use std::thread;

struct MyBox(*mut u8);
unsafe impl Send for MyBox {}

pub fn main() {
    let mut p = MyBox(5 as *mut u8);

    let t = thread::spawn(move || {
        println!("裸指针实现Send {:?}", p.0);
    });

    p = MyBox(6 as *mut u8);

    t.join().unwrap();
}

use std::sync::Arc;
use std::sync::Mutex;
// 裸指针实现Sync
struct SyncBox(*const u8);
unsafe impl Sync for SyncBox {}

pub fn main_2() {
    let b = &SyncBox(5 as *const u8);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || {
        let _v1 =  v.lock().unwrap();
        let ptr = *_v1;
        println!("为裸指针实现Sync 可以在线程中传递{:?}", ptr.0);
    });

    t.join().unwrap();
}