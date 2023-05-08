use std::cell::Cell;
use std::cell::RefCell;

pub fn main() {
  let c = Cell::new("1233");
  let one = c.get();
  c.set("qwer");
  let two = c.get();

  println!("cell 用法{} {}", one, two);
}

pub trait Messenger {
    fn send(&self, msg: String);
}

pub struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        // 解决结构体某个字段变动，而不想设置&mut self的问题
        self.msg_cache.borrow_mut().push(msg)
    }
}

fn main2() {
    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    mq.send("hello, world".to_string());
}