use std::{borrow::BorrowMut, cell::RefCell, rc::{Rc, Weak}};

//
// 构建一个树形数据结构，说明怎么利用 Weak<T> 来解决循环引用的问题
//  
// 一个节点有本身的值，同时有指向父节点的引用和所有子节点的引用
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}


pub fn cycle() {

    // 叶子节点
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // 使用 Weak<T> 的 upgrade() 方法来获取一个 Option<Rc<T>>, 确认引用是否有效
    println!("leaf parent ={:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    // 构建一个作用域
    {
        let branch = Rc::new(Node{
            value: 5, 
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // downgrade 可以增加 Rc 的 weak<T> 引用计数
        // clone 可以增加 Rc 的 strong<T> 引用计数
        // 当所有的强引用计数都被释放时，弱引用自动失效，Rc 将会被销毁  
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        
    }
    // 走出作用域之后再使用leaf 来访问 parent 中的内容
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}

