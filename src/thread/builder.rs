use std::thread;

pub fn run() {
    let handler = thread::Builder::new()
        .name(String::from("Thread 1"))
        // 设置线程栈 (Tie 1 的系统默认的大小为 2 * 1024 * 1024)
        .stack_size(4 * 1024 * 1024)
        .spawn(pick)
        .unwrap();

    handler.join();
}

fn pick(){
    println!("current Thread: {:?}", thread::current().name().unwrap());
}
