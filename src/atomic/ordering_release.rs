use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub fn main() {
    let flag = Arc::new(AtomicBool::new(false));
    let flat_1 = flag.clone();
    let flat_2 = flag.clone();

    let thread1 = thread::spawn(move || {
        // do some work...
        // work is done, now release the flag
        flat_1.store(true, Ordering::Release);
    });

    let thread2 = thread::spawn(move || {
        // wait until the flag is released
        while !flat_2.load(Ordering::Acquire) {
            println!("读取到flag 应该为true，被线程1修改了，Release永远在前面，Acquire在后面");
            // do nothing
        }

        // the flag has been released, do some work...
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
