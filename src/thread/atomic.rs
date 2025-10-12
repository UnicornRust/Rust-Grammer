
// 
// Atomic Type (原子类中的很多操作方法保证了操作的原子性)
// ----------------------------------------------------------------
// > 提供线程之间原始的共享内存通信机制， 是其他并发类型的基础构件
// > 位于  std::sync::atomic 包中以 `Atomic` 开头的类
// > 内部可变性 ：
//      - 允许通过共享引用进行修改
// > 提供了相同的接口
//      - 加载和存储 (load/store) 
//          load  -> 以原子的方式加载原子变量中存储的值
//                -- fn load(&self, order: Ordering)
//          store -> 以原子的方式将新值存入变量
//                -- fn store(&self, val: usize, order: Ordering)
//      - 获取并修改 (fetch-modify)
//                -- fetch_add(value, order) 
//                -- fetch_sub(value, order)
//                -- fetch_and(value, order)
//                -- fetch_or(value, order)
//                -- fetch_xor(value, order)
//                -- ...
//      - 比较和交换 (compare-exchange), 会检查原子值是否等于预期值，如果等于预期值就替换它，否则不做任何更改
//        整个过程是单一原子操作, 会返回之前的值，并且告知是否成功. 
//                -- compare_exchange(&self, current, new, success, failure) -> Result<T, T>
//                   > 方法非常强大，上面介绍的方法都可以使用该方法实现，(循环尝试直到成功, 这种模式叫作 CAS)
//                -- compare_exchange_weak(&self, current, new, success, failure) -> Result<T, T>
//                   > 即使值匹配也可能会失败，在某些平台比 compare_exchange 更高效，如果失败代价较低，rust 更推荐使用这个方法
//                   
// ------------------------------------------------
// > 内存排序是指定在原子操作中如何同步内存
// pub enum Ordering {
//     Relaxed,  // 只保证操作的原子性，不保证顺序(只关心结果，不关系线程操作顺序的场景)
//     Release,
//     Acquire,
//     AcqRel,
//     SeqCst,
// }
//



use std::{ sync::{atomic::{compiler_fence, AtomicBool, AtomicUsize, Ordering}, Arc}, thread, time::Duration};


pub fn run() {
    api()
}

fn api() {
    fence();
    fetch();
    compare();
}




fn compare() {
    let counter = AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0 .. 1000 {
            s.spawn(|| {
                incr(&counter);
            });
        }
    });
    println!("Counter: {}", counter.load(Ordering::Relaxed));
}

fn incr(x: &AtomicUsize) {
    let mut current = x.load(Ordering::Relaxed);
    loop {
        let new = current + 1;
        match x.compare_exchange(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            Err(c) => {
                println!("value changed from {current} to {c}");
                current = c;
            }
        }
    }
}

fn fetch() {
    let done = AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0 .. 10 {
            s.spawn(|| {
                for _ in 0 .. 100 {

                    thread::sleep(Duration::from_millis(29));

                    // 分步操作无法保证原子性
                    // let value = done.load(Ordering::Relaxed);
                    // done.store(value + 1, Ordering::Relaxed);
                    
                    // fetch_add 组合操作保证获取和修改是原子操作
                    done.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
        loop {
            let n = done.load(Ordering::SeqCst);
            if n == 1000 {
                break;
            }
            println!("Process: {n}/1000 done!");
            thread::sleep(Duration::from_millis(100));
        }
    });
    println!("All done");
}


fn fence() {
    // 创建一个 AtomicBool 类型的自旋锁, 通过 Arc 包裹以在多线程之间共享
    let spin_lock = Arc::new(AtomicBool::new(false));

    // 创建两个引用到同一个自旋锁的 clone
    let spin_lock_clone = Arc::clone(&spin_lock);
    let sc = Arc::clone(&spin_lock);

    let thread = thread::spawn(move || {
        // 使用 SeqCst 内存顺序将锁状态设置为 true, 表示锁被占用， 
        // SeqCst 可以确保此操作对所有线程立即可用
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

    // 主线程在这里会持续检查自旋锁的状态, 只要锁的值为 true(被占用), 就会等待
    // 这里也使用 SeqCst 内存顺序来保证锁状态的读取能在多线程中同步
    while spin_lock.load(Ordering::SeqCst) {
        println!("spin_lock c {:?}", spin_lock);
    }

    println!("spin_lock status d {:?}", spin_lock);

    if let Err(e) = thread.join() {
        println!("Thread had an error {:?}", e);
    }
}


