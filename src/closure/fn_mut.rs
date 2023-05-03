pub fn run_fn_mut() {
    let mut a = String::new();
    let b = |x| a.push_str(x);
    exec(b);

    println!("闭包{}", a);
}

// 在 exec 函数中，参数 f 的类型为 FnMut(&'s str) -> ()，这里表示 f 可以被调用多次，
// 每次传入一个 &'s str 的引用。也就是说，闭包 b 可能会被调用多次，每次调用 b 的时候，都需要使用 a 的引用来修改 a 的值。
// 所以，我们需要在 exec 函数中标记 s 生命周期，来表明参数 f 中传入的引用的生命周期必须不短于 exec 函数的生命周期。
// 这样，在 exec 函数中调用闭包 b 的时候，就可以安全地使用 a 的引用来修改 a 的值了。
fn exec<'s, F: FnMut(&'s str) -> ()>(mut f: F) {
    f("abc");
}