pub fn run_iteraotr() {
    let mut f = 2;
    f = 9;
    let mut a = vec![1,2,3];
    let mut i_m_a = a.iter_mut();
    
    if let Some(v) = i_m_a.next() {
        // 解借用
        *v = f;
    }

    println!("输出的vec: {:?} {}", a, f);
}