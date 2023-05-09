use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    // 由于子线程的创建需要时间，因此println!和try_recv方法会先执行，而此时子线程的消息还未被发出。try_recv会尝试立即读取一次消息，因为消息没有发出，此次读取最终会报错，且主线程运行结束(可悲的是，相对于主线程中的代码，子线程的创建速度实在是过慢，直到主线程结束，都无法完成子线程的初始化。。):
    println!("receive {:?}", rx.try_recv());
}

pub fn mian_2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("asda");

        // 若值的类型实现了Copy特征，则直接复制一份该值，然后传输过去，例如之前的i32类型
        // 若值没有实现Copy，则它的所有权会被转移给接收端，在发送端继续使用该值将报错
        tx.send(s).unwrap();

        // 所有权已被转移
        // println!("val is {}", s);
    });

    println!("receive {:?}", rx.recv().unwrap());
}

// 连续接收发送回来的值
pub fn main_3() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let list_str = vec!["123", "456", "789"];

        for item in list_str {
            tx.send(item).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// 多发送者
pub fn main_4() {
    let (tx, rx) = mpsc::channel();

    // 深复制
    // 这里虽然用了clone但是并不会影响性能，因为它并不在热点代码路径中，仅仅会被执行一次
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx.send(String::from("线程1")).unwrap();
    });

    thread::spawn(move || {
        tx2.send(String::from("线程2")).unwrap();
    });

    // 需要所有的发送者都被drop掉后，接收者rx才会收到错误，进而跳出for循环，最终结束主线程
    for received in rx {
        println!("多通道，一个接收者{}", received);
    }
}