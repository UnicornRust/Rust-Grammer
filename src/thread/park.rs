use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, time::Duration};



// 当数据被多个线程修改时，常常会遇到需要等待某个事件，等待数据满足某个条件的情况
//
// - 一种等待其他线程通知的方法叫作线程挂起(thread parking)
//
// > 线程可以将自己挂起（park) 
//     - 这会让线程进入休眠状态（阻塞）， 从而不再消耗 CPU 资源
//     - 之后，另一个线程可以”唤醒“ 被挂起的线程 (unpark) 让它从休眠中醒来。
// > 线程挂起 : std::thread::park()
// > 唤醒线程 : 需要在表示目标线程的 Thread 对象上调用 unpark() 方法
//
// -----------------
//  >> 概念模型 <<  
// -----------------
//  
//  > 每个 Thread handle 都关联一个 token, 初始状态下这个 token 是不存在的
//  > thread::park 会阻塞当前线程，除非或直到 token 对该线程 handle 可用，
//      - 一旦 token 可用，park 会原子性的消耗掉这个 token 并返回
//      - 注意，park 也可能会虚假返回 (spurious wakeup). 即使没有消耗掉令牌，也可能会返回
//  > thread::park_timeout() 与park 类似，但允许指定一个最大的阻塞时间。
//
//  > thread::unpark() 方法会原子性的让 token 可用(如果之前 token 不存在)
//      - 由于初始 token 是缺失的，
//      - 若先调用unpark, 再调用 park, 会使 park 这个调用立即返回
//
//  > 内存顺序 (memeory ordering)
//      - 调用 unpark 会与 park 调用同步（Synchronize-with)
//        这意味着 在调用 unpark 之前完成的所有内存操作, 都可以被消耗该 token 
//        并从 pack 返回的线程看到
//      - 对于同一个线程的所有 park 与 unpark 操作，他们之间形成一个全序关系(total order)
//        并且所有之前的 unpark 操作都与后续的 park 调用同步
//      - 从原子操作的内存序角度
//        unapark 执行的是 release 操作
//        park 执行的是与之对应的 acquire 操作
//        同一个线程上的连续 unpark 调用构成一个 Release 序列
//
// -----------------------
//
pub fn run() {
    api();
}

fn api() {

    let flag = Arc::new(AtomicBool::new(false));
    let flag2 = Arc::clone(&flag);

    let park_thread = thread::spawn(move|| {
        // 测试是否 unpark 之后感知到变量的变化
        while !flag2.load(Ordering::Relaxed) {
            println!("Parking thread");
            // park 的线程会阻塞并让渡 CPU 资源
            thread::park();
            println!("Thread unparked");
        }
        println!("Flag received")
    });

    thread::sleep(Duration::from_millis(100));

    // unpark 之前的内存操作需要被 park 线程感知到
    flag.store(true, Ordering::Relaxed);
    // 通过 handle 来unpark 
    park_thread.thread().unpark();
    park_thread.join().unwrap();

}
