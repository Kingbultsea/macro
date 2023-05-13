use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::sync::Arc;

pub fn main() {
    let var = Arc::new(AtomicI32::new(0));
    let thread1_var = var.clone();
    let thread2_var = var.clone();

    let thread_1 = thread::spawn(move || {
        thread1_var.store(40, Ordering::Release);
    });

    let thread_2 = thread::spawn(move || {
        while thread2_var.load(Ordering::Acquire) == 0 {
            // do something
        }

        thread2_var.store(84, Ordering::Release);
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();

    let value = var.load(Ordering::Acquire);
    println!("var = {}", value);
}