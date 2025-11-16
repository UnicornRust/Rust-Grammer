
//! LayzCell<T, F> 用于懒加载
//! 首次访问时才会执行 F, 后续访问都是直接返回 F 的结果
//! 使用 OnceCell<T> 每次调用 get_or_init 都使用相同的函数
//! LazyCell<T, F> 就适合这种情况
//!    > 把 T 和 F 绑定在一起, 在获得 &T 的时候, 就会执行 F
//!    > 隐式发生，对 LazyCell 解引用，就能会的其内容
//!    > 不是线程安全的
//! -----------------------------------------------------
//! LazyLock<T, F> 用于懒加载
//! 与 LazyCell<T, F> 类似，但是是线程安全的
//!    > 可以在 static 中使用
//!    > 在创建时提供一个无参数的初始化函数来完成初始化
//!    > 由于初始化可能会被多个线程同时调用，如果在另一个初始化过程正在运行的时候调用了解引用操作，那么调用线程会被阻塞
//!   

use std::{cell::LazyCell, sync::LazyLock, thread};


//  
//  如下的对应关系，由于 LazyCell/LazyLock 不太灵活，因此可以使用 OnceCell/OnceLock 替代
//  --- 
//
// LazyCell  -> OnceCell + get_or_init(Fn) 
// LazyLock  -> OnceLock + get_or_init(Fn)
  

pub fn run() {
    api();
    lock();
}


fn api() {
    let lazy = LazyCell::new(init);
    println!("lazy declared: ..");
    println!("get lazy value:  {}", *lazy);
    println!("get lazy value:  {}", *lazy);
}

fn init() -> u32  {
    println!("test init time...");
    32
}

static NUMBER: LazyLock<i32> = LazyLock::new(|| {
    println!("init number ....");
    100
});


fn lock(){
    // 多线程观测数据，第一个获取到的线程会对其进行初始化，其他线程会阻塞
    let handlers: Vec<_> = (0..5).map(|_| {
        thread::spawn(|| {
            println!("Thread see Number: {}", *NUMBER)
        })
    }).collect();

    for h in handlers {
        h.join().unwrap();
    }
}
