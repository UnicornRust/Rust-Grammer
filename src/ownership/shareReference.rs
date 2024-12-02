use std::{rc::Rc, sync::Arc};


// 独占所有权
pub fn exclude() {
    // 独占访问资源
    let mut dynamic_source = String::from("Content");
    let role1: String = dynamic_source;
    // let role2 = dynamic_source; // 资源被 role1 所有，此时 role1 独占访问
    let role2 = role1; // 只有 role1 把 所有权交给 role2, role2 才能访问
    //
    // 这样做的好处就是避免资源被多个变量同时访问，导致资源被修改
    // 坏处是，资源只能被一个变量访问，低效

}

// 所有权与共享容器
pub fn share_container() {
    // 使用共享容器包裹动态资源
    //
    let dynamic_source = vec![1, 2];
    let container = Rc::new(dynamic_source);

    // 这里的 Rc 是引用计数的含义, 或者理解为所有权发生了clone, 而不是资源本身
    let rela1 = container.clone(); 
    let rela2 = container.clone();

    // 通过共享容器访问资源
    // 三个引用都是有效的
    println!("{:?}", container);
    println!("{:?}", rela1);
    println!("{:?}", rela2);


    // 所有权共享容器, 适用与多线程安全操作的情况
    // 使用方式与 RC 一致
    let dynamic_source = String::from("rust");
    let container = Arc::new(dynamic_source);
    let rela1 = container.clone();
    let rela2 = container.clone();

    // 这里资源有三个所有者，都是有效的引用
    println!("{:?}", container);
    println!("{:?}", rela1);
    println!("{:?}", rela2);
    

    // 共享容器与内存管理
    // 注意： Rc<T> 和  Arc<T> 实际上是一种引用技术，
    // 每使用 clone() 方法一次引用计数都会+1,  当变量离开作用域时，引用计数-1，
    // 当引用计数为0时，内存就会被释放, 整个过程每个变量都拥有一个 Rc / Arc, 与所有权规则并不冲突
    // ----
    // 一般情况下，Rust 使用栈来管理堆内存，但是 Rc/Arc 是一种特殊的机制，他允许不受栈内存控制的
    // 堆内存，也就是允许内存泄露，对于这种泄露通用引用计数的方式来管理

    // 4.1  通过栈内存来管理堆内存
    {
        let source = String::from("hello");
        let role = source;
        println!("{:?}", role);
        // 对象走出作用域，栈内存释放的时候, 对应的堆上内存也立即被清理
    }

    // 4.2 通过引用计数来管理内存
    let source = String::from("hello");
    // 使用 Rc 包裹资源，让堆上资源生命周期更长
    let container = Rc::new(source);

    let rela1 = container.clone();
    let rela2 = container.clone();

    // 当变量离开作用域时，rela1, rela2 ，container 相继离开作用域时，引用计数都会-1
    // 当引用计数为 0 时, 堆上的数据才会被释放.
}

