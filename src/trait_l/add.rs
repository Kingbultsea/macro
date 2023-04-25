use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

impl <T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T>{
        Point { x: self.x + p.x, y: self.y + p.y }
    }
}

fn run_add() {
    let p1 = Point{ x: 1, y: 1 };
    let p2 = Point{ x: 3, y: 3 };

    print!("查看默认加法{:?}", p1 + p2);
}
