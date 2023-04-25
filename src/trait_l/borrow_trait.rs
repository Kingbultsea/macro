pub trait People {
    fn my_sex(&self);
}

pub trait Thing {
    fn speak(&self) {
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

impl Thing for WjhLike {}

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

pub fn run_b() -> impl People + Thing {
    let wjh = WjhLike { sex: Sex::Man };
    borrow_trait(&wjh);
    multiple_constraints(&wjh);

    wjh
}

trait Container {
    type A;
    type B;
    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
}

struct ContainerStruct {}

impl Container for ContainerStruct {
    type A = i32;
    type B = i32;

    fn contains(&self, a: &Self::A, b: &Self::B) -> bool {
        false
    }
}

fn difference<C: Container>(container: &C) {}

fn difference2(container: &impl Container) {}

// difference 和 difference2 是等价的，都可以接受实现了 Container trait 的类型作为参数。使用 impl Trait 语法可以更加简洁地表达这一概念。

fn run_container() {
    let a = ContainerStruct{};
    let b = ContainerStruct{};

    difference2(&a);
    difference(&b);
}
