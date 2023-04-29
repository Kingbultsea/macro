struct Interface<'a: 'b, 'b> {
    manager: &'b mut Manager<'a>
}

impl<'a, 'b> Interface<'a, 'b> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str
}

struct List<'a> {
    manager: Manager<'a>,
}


impl<'a> List<'a> {
    // 输入的生命周期和输出的生命周期 一样的， 所以要从'a改成'b
    pub fn get_interface<'b>(&'b mut self) -> Interface<'a, 'b> where 'a: 'b {
        Interface {
            manager: &mut self.manager
        }
    }
}

fn run_end_test() {
    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了
    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
