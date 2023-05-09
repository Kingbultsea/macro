// 所有发送者被drop或者所有接收者被drop后，通道会自动关闭

pub mod mpsc;
pub mod thread;
pub mod try_recv;