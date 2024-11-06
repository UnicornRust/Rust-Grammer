//

use std::{cell::RefCell, rc::Rc};
use List::{Cons, Nil};
use Cycle::{Crons, Null};



#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}


// 基于 Rc 和 RefCell 实现多重所有权的可变数据
pub fn run() {

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}


    
// 循环引用
#[derive(Debug)]
enum Cycle {
    Crons(i32, RefCell<Rc<Cycle>>),
    Null
}

impl Cycle {
    fn tail(&self) -> Option<&RefCell<Rc<Cycle>>> {
        match self {
            Crons(_, item) => Some(item),
            Null => None
        }
    }
}

pub fn cycleRef() {

    let a = Rc::new(Crons(5, RefCell::new(Rc::new(Null))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Crons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);   
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack 
    // println!("a next item = {:?}", a.tail());

}

