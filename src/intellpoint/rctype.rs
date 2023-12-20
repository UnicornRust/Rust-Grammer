use crate::intellpoint::rctype::List::{Cons, Nil};
use std::rc::Rc;

pub fn run() {
    rc_type();
    rc_type_refer_check();
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// rc 引用计数指针指针得引入场景）
fn rc_type() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // Rc::clone() 函数需要 a 中的 Rc<List> 的引用作为参数
    // 值得注意的是，这里的 Rc::clone 的实现并不会像大部分类型的 clone 实现那样是一个数据的深拷贝
    // Rc::clone 只会增加引用计数，这并不会花费多少时间
    let _b = Cons(3, Rc::clone(&a));
    let _c = Cons(4, Rc::clone(&a));
}

fn rc_type_refer_check() {
    // 我们可以使用 Rc::strong_count(&a) 来观测数据的引用计数的变化
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after create a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count agter create b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after create c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a))
}
