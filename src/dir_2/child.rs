use crate::abc::use_abc;

pub fn child() {
    println!("child is me");

    // 二进制入口里 已经有mod abc了
    use_abc();
}