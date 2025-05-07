use std::thread;

pub fn run() {
    create_thread();
    scope_thread();
}

// 线程创建的方法啊
fn create_thread() {

    let person = Person {
        id : 1, 
        name : String::from("hello"),
    };

    // 传递一个闭包作为需要执行的任务, 得到一个 JoinHandle 作为线程的句柄
    // 如果想要持有外部作用域的变量， 需要使用 move 关键字(线程执行的时机是不确定的，因此需要获取到所有权)
    let handle = thread::spawn(move || {
        println!( "this is {} to run code, no: {:?}",
            thread::current().name().unwrap(), 
            person
        )
    });

    // 需要等待线程执行完毕, 主线程可能执行完成之后整个程序的生命周期结束
    // 这里需要保证开启的后台线程执行完成再结束 main 线程
    handle.join().unwrap();
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
}

fn scope_thread() {

    let person = Person {
        id : 1, 
        name : String::from("hello"),
    };

    // scope 会确保再 main 线程结束前, 所有 scope 内的线程执行完成
    // 因此可以在 scope 中可以持有借用 person 
    thread::scope(|scope|{
        scope.spawn(||{
            println!("scope:: {:?}", person);
        });
    });
}
