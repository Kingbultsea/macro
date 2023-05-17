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

pub struct WjhLike {
    sex: Sex,
}

pub struct ASD {
    sex: Sex,
}

impl Thing for WjhLike {}

impl People for WjhLike {
    fn my_sex(&self) {
        println!("my sex is: {:?}", self.sex);
    }
}

impl People for ASD {
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

pub fn run_b() -> WjhLike {
    let wjh = WjhLike { sex: Sex::Man };
    let asd = ASD { sex: Sex::Man };
    borrow_trait(&wjh);
    borrow_trait(&asd);
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

// difference 和 difference2 是等价的，都可以接受实现了 Container trait 的类型作为参数。
// 使用 impl Trait 语法可以更加简洁地表达这一概念。

fn run_container() {
    let a = ContainerStruct {};
    let b = ContainerStruct {};

    difference2(&a);
    difference(&b);
}

trait Container2<A, B> {}

// where C: Container<A, B> 的作用是指定泛型类型 C 必须实现 Container2<A, B> 这个特征。它约束了泛型类型的实现，
// 使得只有实现了 Container<A, B> 的类型才能作为参数传递给 difference3 函数
fn difference3<A, B, C>(container: &C)
where
    C: Container2<A, B>,
{
}

// 元组结构体通常用于只有少量字段的简单结构，其中每个字段的名称并不重要。
// 与标准结构体相比，它们更容易编写和阅读，因为不需要指定字段名。
struct Meter(i32);
