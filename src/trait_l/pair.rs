use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// PartialOrd是一个trait，它表示类型可以进行部分比较。部分比较是指可以进行小于、
// 大于和等于的比较，但是对于某些值无法确定大小关系（如NaN）
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn run_pair() {
    let a = Pair{ x: 1, y: 2 };
    a.cmp_display();
}
