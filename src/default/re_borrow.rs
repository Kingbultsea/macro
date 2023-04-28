#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn re_borrow() {
    let mut origin = Point{ x: 1, y: 2 };
    let r = &mut origin;

    // 再借用，不属于多重借用
    let rr = &*r;
    println!("{:?}", rr);

    // rr 借用结束，可以再使用r
    r.x = 9;
    println!("{:?}", r);

    // r 借用结束，可以再使用r
    origin.x = 5;
    println!("{:?}", origin);
}