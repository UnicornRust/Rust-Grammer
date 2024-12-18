
// 闭包实现与 trait
// 在Rust中，闭包实际上是一个语法糖, 它的实现在抽象概念上可以看做是一个匿名结构体
// 这个结构体会把环境变量捕获称为其成员，并实现 Fn / FnMut / FnOnce trait
// 而 Fn / FnMut / FnOnce 中各有一个方法 call() / call_mut() / call_once()
// 对应的语义分别是调用不可变闭包，调用可变闭包，调用单次(消费)闭包
// 并且 Fn : FnMut : FnOnce 之间有依次继承的关系链，因此 fn 满足 FnMut 和 FnOnce trait
//
// 当调用一个闭包的时候，编译器会根据闭包的类型，自动推导出其实现的 trait, 一般情况下不需要手动实现
//


// 闭包作为参数传递，参数形式类似与 trait bound 的写法, 这是独属于闭包的写法

fn call_fn<F: Fn()>(f: F) {}

fn call_fn_mut<F: FnMut()>(f: F){}

fn call_fn_once<F: FnOnce()>(f: F) {}


// 闭包的调用

fn use_closure() {

    // 未捕获环境
    let c = || println!("closure");

    call_fn(c);
    call_fn_mut(c);
    call_fn_once(c);

    // 捕获了不可变引用
    let x = "10";
    let c = || println!("get env var: {}", x);

    call_fn(c);
    call_fn_mut(c);
    call_fn_once(c);

    // 捕获可变的引用
    let mut x = String::from("20");
    let mut c = || {
        x.push_str("20");
        println!("get mut env var {:?}", x);
    };

    // call_fn(c);
    // call_fn_once(c);
    call_fn_mut(c);
    
    
}

// 闭包作为返回值
// 写法与 trait bound 的写法也是类似的
fn return_fn() -> impl Fn() {
    || println!("closure")
}

fn return_fn_mut() -> impl FnMut() {
    let x = 10;
    // || println!("call_fn_mut", x + 1) // 不能返回局部变量，必须要移入闭包才能返回
    move || println!("call_fn_mut: {}", x )
}

fn return_fn_once() -> impl FnOnce() {
    let s: String = String::from("hello");
    // 必须将变量移入闭包才能正常返回
    move || println!("call_fn_once: {}", s) 
}

