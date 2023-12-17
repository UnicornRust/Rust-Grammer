use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn run() {
    mutex_api();
    multiple_thread_with_mutex();
}

// 从单线程上下文认识互斥器
fn mutex_api() {
    // 使用 mutex 的关联方法获取 Mutex<T> 对象
    let m = Mutex::new(5);

    {
        // 使用 mutex 中使用 lock() 方法获取锁
        // 以访问互斥器中的数据，这个调用会阻塞房钱线程，直到拥有锁为止
        // 如果另一个线程拥有锁并且那个线程panic了，那么这个lock() 调用将会失败
        // 并不会有线程再获取到 lock, 所以选择 unwrap() 在这种情况直接panic
        let mut num = m.lock().unwrap();
        *num = 7;
    }
    println!("m = {:?}", m);
}

//多线程之间通过mutex<T> 共享值
fn multiple_thread_with_mutex() {
    // 启动十个线程对同一个计数器尝试加1
    // 使用Arc<T> 来创建引用计数的值，以便拥有多所有者，
    let counter = Arc::new(Mutex::new(0));

    // 收集线程
    let mut handles = vec![];

    for _ in 0..10 {
        // 移入线程之前克隆 Arc<T>
        let counter = Arc::clone(&counter);

        // 这里每个构建的线程都调用 lock 方法获取哦 Mutex<T> 上的锁.
        // 接着将互斥器中的值加1，
        // 当一个线程结束执行，num 会离开闭包作用域并释放锁，
        // 这样另一个线程就可以获取锁继续运行
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // 调用所有线程的 join 方法，保证所有的线程都运行结束.
    for handle in handles {
        handle.join().unwrap();
    }
    // 等到所有新建的线程运行结束，主线程答应结果.
    println!("Result: {}", *counter.lock().unwrap());
}
