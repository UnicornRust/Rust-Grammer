
// 
// 条件变量 (Condvar)
//    - 提供在等待事件发生时阻塞线程的能力
//    - 表示能够阻塞一个线程的能力，使其在等待事件发生时不消耗 CPU 时间
//    - 条件变量通常与一个 bool 谓词（predicate 一个条件) 和一个 mutex 关联
//    - 在决定线程必须阻塞时，谓词总是在 mutex 内部被验证
// > 注意： 任何试图在同一个 Condvar 上使用多个 Mutex 的操作，可能会导致运行时的 panic 
//

use std::{sync::{Arc, Condvar, Mutex}, thread, time::Duration};


pub fn run() {
    api();
}

fn api(){ 

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // 通知一个阻塞当前变量的线程
        thread::sleep(Duration::from_millis(1000));
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();

    println!("listen： started: {}", *started);
    // 阻塞当前线程直到这个条件改变,
    // 此时获取到这个条件的值，然后退出循环
    started = cvar.wait(started).unwrap();
    println!("active: {}", *started);
}


