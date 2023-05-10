use std::sync::{Arc, Mutex};
use std::thread;

pub fn main() {
    let v = Mutex::new(5);

    {
        // v.lock()向v申请一个锁, 该方法会阻塞当前线程，直到获取到锁
        // 因此当多个线程同时访问该数据时，只有一个线程能获取到锁，其它线程只能阻塞着等待，这样就保证了数据能被安全的修改
        let mut a = v.lock().unwrap();

        // 它实现了Deref特征，会被自动解引用后获得一个引用类型，该引用指向Mutex内部的数据
        // 它还实现了Drop特征，在超出作用域后，自动释放锁，以便其它线程能继续获取锁
        *a = 6;
    }

    println!("m = {:?}", v);
}

pub fn main_2() {
    let m = Mutex::new(5);

    let mut num = m.lock().unwrap();
    *num = 6;
    // 锁还没有被 drop 就尝试申请下一个锁，导致主线程阻塞
    drop(num); // 手动 drop num ，可以让 num1 申请到下个锁

    let mut num1 = m.lock().unwrap();
    *num1 = 7;
    drop(num1); // 手动 drop num1 ，观察打印结果的不同

    println!("m = {:?}", m.into_inner().unwrap());
}

pub fn main_3() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for a in 0..10 {
        // println!("{}", a); 0..10是不包括10的
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}