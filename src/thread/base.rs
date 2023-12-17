use std::{sync::mpsc, thread, time::Duration};

pub fn thread() {
    create();
    channel();
    iter_recv();
}

// 通道可以遍历的特性, 方便我们在线程之间通信
fn iter_recv() {
    let (tx, rx) = mpsc::channel();

    // 将发送者移动到新的线程中
    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("world"),
            String::from("new"),
            String::from("from"),
            String::from("thread"),
        ];
        for val in vals {
            // send 会获取数据的所有权，将数据发送出去，同时将所有权移动
            // 到另一个线程，后续不能在当前线程继续使用
            tx.send(val).unwrap();
            // 每发送一个休眠 1s
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 这里我们的接收者不再显式调用 recv 函数，而是将 rx 当作一个迭代器，
    // 对于每一个一个接收到的值, 我们将其打印出来.
    // 当信道被关闭时，迭代气也将结束。
    for received in rx {
        println!("Got: {received}");
    }

    handle.join().unwrap();
}

// 线程之间通信交换信息
fn channel() {
    // mpsc 是多生产者单消费者, 返回一个元组，使用 let 解构
    // 获得 transmitter 和 receiver, tx 和 rx 是简写
    let (tx, rx) = mpsc::channel();

    // 将发送者移动到新的线程中
    let handle = thread::spawn(move || {
        let val = String::from("hi");
        // send 会获取数据的所有权，将数据发送出去，同时将所有权移动
        // 到另一个线程，后续不能在当前线程继续使用
        tx.send(val).unwrap();
    });

    // 接收者游两个方法可以接收对应的数据
    // recv: 阻塞接受者线程，直到有一个数据,
    //    - 数据以Result<T, E> 形式返回
    //    - 当信道关闭接收到一个错误表示不会有新值到来.
    // try_recv: 不会阻塞当前接收者线程，直接返回，
    //    - 当有数据时 Ok 值包含可用的信息，
    //    - Err 则表示此时没有消息.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    // 等待程序执行完成
    handle.join().unwrap();
}

// 线程创建的方法啊
fn create() {
    thread::spawn(|| {
        for i in 1..10 {
            println!(
                "this is {} to run code, no: {i}",
                thread::current().name().unwrap()
            )
        }
    });
}
