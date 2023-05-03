pub fn run_closure() -> i32 {
    let x = 1;

    // 自动推断类型
    // 当编译器推导出一种类型后，它就会一直使用该类型
    let sum = |y| x + y;

    sum(1)
}
