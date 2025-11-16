//! MPSC (Multiple Producer, Single Consumer) Channel
//! 基于消息的通道 (channel) 通信机制
//!
//! 三种类型 ()
//!   > Sender
//!   > SyncSender
//!   > Receiver
//!
//! 通道的类型 
//!
//!  > 异步通道 (asynchronous channel) 无限缓冲区的通道
//!      - channel  函数返回一个 Sender 和一个 Receiver
//!      - 发送操作是异步的
//!      - 概念上拥有无限容量的缓冲区
//!
//!  > 同步通道 (synchronous channel) 有界缓冲区的通道
//!      - sync_channel 函数返回一个 (SyncSender, Receiver) 
//!      - 其内部为预先分配好的固定大小的缓冲区
//!      - 发送操作都是同步的, 除非缓冲区还有容量，否则就会阻塞
//!      
//!  ** 注意: 缓冲区大小允许设置为 0, 通道变成一种会和(rendezous) 通道
//!     - 这意味着每个发送者都会原子的将消息直接发送给接收者
//!
//!  > 断开连接(Disconnection) 
//!
//!     - 对通道的 `send` 和 `receive` 操作都会返回一个 `Result` 
//!       用以指示操作是否成功, 
//!     - 如果操作失败，通常表示通道的另一端已经 `挂起` 或被释放(drop)
//!     - 一旦通道的一半被释放，大多数的操作将无法继续进行，因此会返回`Err`
//!        * 在许多的应用中，开发者会继续对这些 Result 调用 `unwrap` 
//!          从而在某个线程意外终止时，引发错误在其他线程中传播.
//!        * 
//!

use std::{sync::mpsc, thread};


type Task = Box<dyn FnOnce() + Send + 'static>;


enum Msg {
    Call(Task),
    Exit,
}


fn hello() {
    println!("hello world");
}

pub fn run() {

    let (tx, rx) = mpsc::channel::<Msg>();
    let handle = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            match msg {
                Msg::Call(task) => task(),
                Msg::Exit => break
            }
        }
    });

    let  closure = || println!("hello closure");

    tx.send(Msg::Call(Box::new(hello))).unwrap();
    tx.send(Msg::Call(Box::new(closure))).unwrap();
    tx.send(Msg::Call(Box::new(|| println!("hello Box new")))).unwrap();
    tx.send(Msg::Exit).unwrap();

    handle.join().unwrap();
}


