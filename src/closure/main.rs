pub fn run_closure() -> i32 {
    let x = 1;

    // 自动推断类型
    // 当编译器推导出一种类型后，它就会一直使用该类型
    let sum = |y| x + y;
    let cal = factory(10);

    println!("闭bao后的数值：{}", cal(1));

    sum(1)
}

fn factory(num: i32) -> Box<dyn Fn(i32) -> i32> {
    if num > 20 {
        // 闭包取得捕获变量的所有权 move
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}