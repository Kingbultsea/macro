use std::sync::Mutex;

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