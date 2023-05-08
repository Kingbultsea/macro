use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

// 主人
struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

// 工具
struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

pub fn main() {
    run_weak();
    println!("panicked ----");
}

pub fn run_weak() {
    let owner = Rc::new(Owner{ name: String::from("value"), gadgets: RefCell::new(Vec::new()) });

    let gadget1 = Rc::new(Gadget{ id: 1, owner: owner.clone() });
    let gadget2 = Rc::new(Gadget{ id: 2, owner: owner.clone() });

    // weak
    owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1));
    owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2));

    // 尝试让指引消失
    // drop(gadget1);
    drop(gadget2);

    for gadget_opt in owner.gadgets.borrow().iter() {
        // gadget_opt 是一个 Weak<Gadget> 。 因为 weak 指针不能保证他所引用的对象
        // 仍然存在。所以我们需要显式的调用 upgrade() 来通过其返回值(Option<_>)来判断其所指向的对象是否存在。
        // 当然，Option 为 None 的时候这个引用原对象就不存在了。

        // 解决panicked
        if let Some(gadget) = gadget_opt.upgrade() {
            println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
        }

        // let gadget = gadget_opt.upgrade().unwrap();
        // println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }
}