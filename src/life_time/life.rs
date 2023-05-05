fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

pub fn run_life() {
    let first = String::from("a");
    let second = String::from("a");
    let longest_num = longest(first.as_str(), second.as_str());

    println!("最大值为{}", longest_num);

    life_test();

    funny();
}

pub fn life_test() {
    let string1 = String::from("a");

    {
        let string2 =  String::from("a");
        let longest_num = longest(string1.as_str(), string2.as_str());
        // longest_num的生命周期 和 string2一样，取最短

        println!("最大值为 2: {}", longest_num);
    }
}

// 该生命周期标注说明，结构体 ImportantExcerpt 所引用的字符串 str 必须比该结构体活得更久
struct ImportantExcerpt<'a> {
    part: &'a str
}

fn run_import_excerpt() {
    let a = String::from("asdasd");

    let c = ImportantExcerpt{
        part: a.as_str()
    };
}

// 'a: 'b,的意思就是a必须要比b活得久
struct ImportantExcerpt2<'a> {
    part: &'a str
}
impl<'a: 'b, 'b> ImportantExcerpt2<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
// 
// impl<'a, 'b> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str 
//     where
//         'a: 'b,
//     {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }
// 
// fn main() {
//     let s0 = "one";
//     let s2;
//     {
//         let s1 = "two";
//         let st = ImportantExcerpt{part: s1};
//         s2 = st.announce_and_return_part(s0);  // error: borrowed value does not live long enough
//     }  // 'a 结束 `st` dropped here while still borrowed
//     println!("{}", s2);  // borrow later used here
// }  // 'b 结束

#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share<'a>(&'a mut self) -> &'a Self {
        // mutate_and_share 方法中没有输入生命周期。但是，因为它是一个可变借用，它会使用当前实例的所有权，
        // 并在方法调用结束后将它归还给原始变量。在这种情况下，Rust 会使用默认的生命周期 'a，该生命周期的作用域由 &mut self 的生命周期限制，即该可变借用的生命周期。
        // 因此，该方法的签名可以写作 fn mutate_and_share<'a>(&'a mut self) -> &'a Self，其中 'a 是输入生命周期，表示方法中可变引用的生命周期。
        &*self
        // 想要解决问题 就直接返回self吧
    }
    fn share(&self) {}
}

fn funny() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    // foo.share(); 不能再进行不可变借用了
    println!("{:?}", loan);
}
