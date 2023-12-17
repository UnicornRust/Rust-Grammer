use std::thread;

pub fn run() {
    create();
}

// 线程创建的方法啊
fn create() {
    thread::spawn(|| {
        for i in 1..10 {
            println!(
                "this is {} to run code, no: {i}",
                thread::current().name().unwrap()
            )
        }
    });
}
