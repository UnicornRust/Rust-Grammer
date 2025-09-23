use std::{
    sync::{Arc, Mutex},
    thread,
};

use libc::personality;


// mutex 是  mutunal exclusive 互斥器原语(primitive)
//  - lock   可以加锁的的状态 
//  - locked 已经加锁的状态
//  - poisoned (中毒的) 由某一个访问锁的线程panic 导致
//    > 调用 lock/ try_lock 会返回一个 Result<T, PoisonError<T>>
//      用于指示该互斥锁是否中毒，
//    > 中毒的 Mutex 不会阻止对底层数据的访问，而是提供了一个 into_inner()
//      方法来消耗掉这个互斥锁，可以返回原本在加锁时的守卫对象，可以通过
//      这个守卫对象来访问互斥锁中的数据


pub fn run() {
    // mutex_api();
    // multiple_thread_with_mutex();
    poisoned_mutex();
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
    // Arc<T> 与 Rc<T> 之间的区别是可以用于并发环境, 确保了操作的原子性
    // 因为在多线程场景下, Rc<T> 无法保证引用的增加与减少是原子操作，
    // 在计数出现错误的时候回出现诡异的 bug, 内存泄漏/野指针
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


// mutex poisoned

fn poisoned_mutex() {

    let data = Arc::new(Mutex::new(10));
    {
        // 触发异常
        let data = Arc::clone(&data);
        thread::spawn(move||{ 
            let mut num = data.lock().unwrap();
            *num += 1;
            panic!("error to handle");
            // unwrap_err() 已经预知了发生的错误并进行了处理恢复
        }).join().unwrap_err();
    }
    // 另一个线程尝试获取锁进行操作的时候会得到poisoned错误, 并且可以此线程中进行恢复
    {
        let data = Arc::clone(&data);
        thread::spawn(move|| {
            match data.lock() {
                Ok(mut guard) => {
                    println!("Thread Ok: {guard}",);
                    *guard += 10;
                },
                Err(poisoned) => {
                    println!("Thread Poisoned: {poisoned} ");
                    // 从错误中可以恢复 guard 并继续使用
                    let mut guard = poisoned.into_inner();
                    *guard += 100;
                    println!("Thread new Value: {}", *guard);
                }
            }
        }).join().unwrap();
    }
}
