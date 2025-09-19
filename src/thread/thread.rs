use std::thread;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
}

pub fn run() {
    create_thread();
    scope_thread();
    chunk_sum();
}

// 线程创建的方法啊
fn create_thread() {
    let person = Person {
        id: 1,
        name: String::from("hello"),
    };
    // 传递一个闭包作为需要执行的任务, 得到一个 JoinHandle 作为线程的句柄
    // 如果想要持有外部作用域的变量， 需要使用 move 关键字(线程执行的时机是不确定的，因此需要获取到所有权)
    let handle = thread::spawn(move || {
        println!(
            "this is {} to run code, no: {:?}",
            thread::current().name().unwrap_or("default"),
            person
        )
    });
    // 需要等待线程执行完毕, 主线程可能执行完成之后整个程序的生命周期结束
    // 这里需要保证开启的后台线程执行完成再结束 main 线程
    handle.join().unwrap();
}

//
// 作用域线程中的两个生命周期
//
// 1.'env 是代表被作用域线程借用的那些数据的生命周期，这个生命周期
//    必须要长于整个 thread::scope() 的调用的, 最少也要等于这个
//    scope 调用时间 (决定了那些数据能被借用)
//
// 2.'scope 是在传入thread::scope(||{}) 的闭包开始之前生效
//    在内部闭包运行结束， scope 运行结束之前结束.
//    (控制着作用域线程在作用域内执行结束)
//

fn scope_thread() {
    let person = Person {
        id: 1,
        name: String::from("hello"),
    };
    // scope 会确保在作用域结束前, 所有 scope 内的线程执行完成
    // 因此可以在 scope 中可以持有借用 person
    // 这里借用的对象的生命周期一定是 >= scope
    //
    thread::scope(|scope| {
        //
        // 如果这里有一个变量，是无法借用到 spawn 函数中的
        // 因为他的生命周期是要 <= scope 的,
        //
        scope.spawn(|| {
            println!("scope:: {:?}", person);
        });
    });
}

fn chunk_sum() {
    const CHUCK_ZISE: usize = 10;
    let numbers: Vec<u32> = (1..10000).collect();
    let chunks = numbers.chunks(CHUCK_ZISE);

    let sum: u32 = thread::scope(|s| {
        let mut handlers = Vec::new();
        for chuck in chunks {
            let h = s.spawn(|| chuck.iter().sum::<u32>());
            handlers.push(h)
        }

        handlers.into_iter().map(|h| h.join().unwrap()).sum()
    });
    println!(" 1.. 10000 sum is {sum}")
}
