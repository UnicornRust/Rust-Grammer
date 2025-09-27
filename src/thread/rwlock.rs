
// 读写锁 std::sync::RwLock<T>
//   > 可以同时持有多个读锁，但是只能有一个写锁
//   > 适用于经常被多线程读取，偶尔更新的场景
//   > T 必须是 Send(可以在线程之间移动) + Sync(安全的在线程共享引用)
// 存在三种锁的状态
//   - 未锁定
//   - 由独占的写入者锁定
//   - 由任意数量的读者锁定
// > read()  --> RwLockReadGuard(Deref)
// > write() --> RwLockWriteGuaard(Deref & DerefMut)

//  std 的 RwLock: 具体的实现与操作系统相关
//    - (大部分都是写入优先) 会先阻断新的读取者，即使当前是读取锁定的状态
//      因为在一个读多写少的场景下，写入者可能总也抢不到所，但是我们又希望
//      更新操作能尽快执行。

use std::{sync::{Arc, RwLock}, thread::{self, Thread}};


// RwLock 的中毒
//    > 与 Mutex 类似，RwLock 在发生 panic 的时候也可能进入中毒状态，但是不同
//      的是只有在写锁的时候发生 panic 时会中毒，而在读锁的时候不会中毒。
//
pub fn run() {
    use_rw();
}

fn use_rw() {
    let counter = Arc::new(RwLock::new(0));
    let mut handler = Vec::new();

    for i in 0..10 {
        let co = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let value = co.read().unwrap();
            println!("Thread: {} read  value: {}", thread::current().name().unwrap_or_default(), value);
        });
        handler.push(h);
    }
    {
        let co = Arc::clone(&counter);
        let h = thread::spawn(move|| {
            let mut value = co.write().unwrap();
            *value += 1;
            println!("Write update the value : {value}")
        });
        handler.push(h);
    }

    handler.into_iter().for_each(|h| h.join().unwrap());

    let value = counter.read().unwrap();
    println!("Writer: {}", value);
}

