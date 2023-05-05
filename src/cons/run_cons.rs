enum List {
    Cons(i32, Box<List>),
    Nil
}

fn println_list(list: &List) {
    match list {
        List::Cons(val, next) => {
            println!("value: {}", val);
            println_list(next);
        },
        List::Nil => println!("Nil"),
    }
}

pub fn run_rs() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println_list(&list);
}