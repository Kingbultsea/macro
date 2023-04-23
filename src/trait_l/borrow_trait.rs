trait People {
    fn my_sex(&self);
}

trait Thing {
    fn speak (&self) {
        println!("i can speak");
    }
}

#[derive(Debug)]
enum Sex {
    Man,
    Woman,
}

struct WjhLike {
    sex: Sex,
}

impl Thing for WjhLike{}

impl People for WjhLike {
    fn my_sex(&self) {
        println!("my sex is: {:?}", self.sex);
    }
}

fn borrow_trait(item: &impl People) {
    item.my_sex();
}

fn multiple_constraints(item: &(impl People + Thing)) {
    item.my_sex();
    item.speak();
}

pub fn run_b() {
    let wjh = WjhLike{ sex: Sex::Man };
    borrow_trait(&wjh);
    multiple_constraints(&wjh);
}