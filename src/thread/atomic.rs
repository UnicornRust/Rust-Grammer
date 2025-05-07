
// 使用原子类型共享数据
//

use std::{ sync::{atomic::{compiler_fence, AtomicBool, Ordering}, Arc}, thread, time::Duration};


pub fn run() {

    // 创建一个 AtomicBool 类型的自旋锁, 通过 Arc 包裹以在多线程之间共享
    let spin_lock = Arc::new(AtomicBool::new(false));

    // 创建两个引用到同一个自旋锁的 clone
    let spin_lock_clone = Arc::clone(&spin_lock);
    let sc = Arc::clone(&spin_lock);

    let thread = thread::spawn(move || {
        // 使用 SeqCst 内存顺序将锁状态设置为 true, 表示锁被占用， SeqCst
        // 可以确保此操作对所有线程立即可用
        // 即无论其他线程在何处，他们都能看到这个改变
        spin_lock_clone.store(true, Ordering::SeqCst);
        println!("spin_lock status a {:?}", sc);

        // 休眠 2s
        let time = Duration::from_secs(2);
        thread::sleep(time);

        // 设置编译器栅栏，内存顺序是 Release, 这意味着这个栅栏之前的所有操作
        // (包括上面的 println!, 和 sleep) 都会在这个栅栏之前完成.
        // Release 语义：保证在此之前的操作都先执行完毕，确保在你更改共享数据之前
        // 所有其他的线程对这个数据的的引用都已经完成。

        compiler_fence(Ordering::Release);

        // 使用 SeqCst 内存顺序将锁状态设置为 false, 表示锁已经释放，
        // SeqCst 可以保证这个操作对所有线程立即可见
        
        spin_lock_clone.store(false, Ordering::SeqCst);
        println!("spin_lock status b {:?}", sc);

    });

    // 主线程在这里会持续检查自旋锁的状态, 只要所的值为 true(被占用), 就会等待
    // 这里也使用 SeqCst 内存顺序来保证锁状态的读取能在多线程中同步
    while spin_lock.load(Ordering::SeqCst) {
        println!("spin_lock c {:?}", spin_lock);
    }

    println!("spin_lock status d {:?}", spin_lock);

    if let Err(e) = thread.join() {
        println!("Thread had an error {:?}", e);
    }
}
