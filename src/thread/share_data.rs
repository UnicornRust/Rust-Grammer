use std::{sync::Arc, thread};

pub fn share() {
    static_data();
    leak();
    arc();
}

/// 线程间共享数据的方式
///
/// 1. static : 
///   > 在整个程序的生命周期内有效, 实际就是编译在二进制文件中的数据
///   > 都是使用常量值来初始化，
///   > 代表了一个内存地址，可以进行引用
///   > 在程序结束的时不会调用 drop 方法 
///   > 可以是不可变的，也可以是可变的(mut 修饰即可)
///   > 
///
fn static_data() {
    
}

// 2. Box::leak()
//   > 释放 Box 的所有权，并承诺永远不会 drop 它
//   > 从 leak 这一刻开始，这个 Box 就一直存在了
//   > 因为没有所有者，只要程序运行就可以被任何线程借用
//
// 缺点: 它的本质是内存泄露，一个程序里面不要过渡使用.

fn leak() {
    // data 类型标注 ('static [i32;5]) 的必要性 
    // leak() 函数返回的是一个 &'a T 的引用, 如果不手动标注其为 &'static T 
    // 则编译器无法推断其为 'static, 也就无法跨线程进行借用
    let data: &'static [i32;5] = Box::leak(Box::new([1, 2, 3, 4, 5]));
    let mut handles = Vec::new();
    for  _ in 0..10000 {
        // move 的含义
        //  这里使用 move 移动的语义，并没有真正的移动 data 到新的线程
        //  因为 data 在这里是一个`'static` 生命周期的引用，其实就是
        //  将这个引用的所有权转移给了新的线程(相当于复制了一个引用并移动了所有权)
        let h = thread::spawn(move || {
            println!("Data: {data:?}");
        });
        handles.push(h);
    }
    handles.into_iter().for_each(|h| { h.join().unwrap()});
}

// 3. Arc<T> 原子引用计数(atomically reference counted)
//   > 与 Rc<T> 类似，但 Arc<T> 保证对引用计数器的修改是不可分割的原子操作
//   > 可以在多线程环境中使用

fn arc() {
    let data = Arc::new([1, 2, 3, 4, 5]);
    let mut handles = Vec::new();

    for _ in 0..10000 {
        // 每次使用 Arc<T> 克隆一个新的引用, 引用计数 +1 
        // 走出作用域时, 引用计数 -1
        let local_data = data.clone();
        let h = thread::spawn(move || {
            println!("Data: {local_data:?}");
        });
        handles.push(h);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
}
