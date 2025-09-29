//
// Refcell<T> 
//
//------------------------------------------------------------
// 1. 与 Cell<T> 不同，RefCell<T> 允许直接借用内部值
//   - 有一点性能开销
// 2. RefCell<T> 不仅持有T的值，还持有一个计数器, 用来追踪有多少个借用
//   - 借用是在运行时被追踪的
//   - Rust 原生的引用类型则完全是在编译时静态检查的
// -----------------------------------------------------------
//
// >> 常用方法
//
// 1. borrow() 方法获取 RefCell 内部值的不可变借用
// 2. borrow_mut() 方法获取对 RefCell 内部值的可变引用(&mut T)
//    > 调用上述两个借用方法时，
//      - 首先会检查借用规则(任意个不可变借用或者单个可变借用)
//      - 如果不满足借用规则，就会 panic
// 3. 其他
//    - try_borrow() 和 try_borrow_mut() 返回的是 Result，不会在已经
//      存在可变借用时再次调用了 borrow() / borrow_mut() 时出现 panic
//    - into_inner() 方法，消耗(consume) 掉这个RefCell<T>, 并返回内部值
//    - replace() 方法替换当前内部值，返回原来的值
//    - take() 方法会将当前内部值替换为 Default::default(),并返回原来值
// ------------------------------------------------------------
//
//
use std::cell::RefCell;

struct MockMessage {
    sent_messages: RefCell<Vec<String>>,
}

impl Message for MockMessage {
    fn send(&self, msg: &str) {
        // 借用可变
        self.sent_messages.borrow_mut().push(msg.to_string());
    }
}
    
pub fn run() {
    api();
    tracer();
}

fn tracer() {

    let mock_message = MockMessage {
        sent_messages: RefCell::new(vec![]),
    };
    let mut limit_tracker = LimitTracker::new(&mock_message, 100);
    limit_tracker.set_value(80);

    // 此处使用 into_inner 消耗掉 RefCell，后续不可继续使用
    let message = mock_message.sent_messages.into_inner();
    for m in message {
        println!("message: {m}");
    }
    // println!("{:?}", mock_message.sent_messages.borrow().len());
}

fn api() {
    let rc = RefCell::new(5);
    {
        // 多个不可变借用
        let _ = rc.borrow();
        let _ = rc.borrow();
        println!("{rc:#?}");
    }
    // 不可变借用
    let mut f = rc.borrow_mut();
    // 
    *f += 6;
    drop(f);

    // 此写法与上述的写法是等效的
    // borrow_mut 返回 RefMut, RefMut 实现了DeRef,
    // 因此在下面的写法中可以直接使用 *来解引用获得可变借用
    // 这个可变借用在表达式结束之后就被丢弃了
    *rc.borrow_mut() += 6;
    // 这里无须 drop
    //
    println!("{rc:#?}");
}


pub trait Message {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Message> {
    messager: &'a T,
    value: usize,
    max: usize,
}


//
// 内部可变性
//
impl<'a, T> LimitTracker<'a, T> 
where 
    T: Message 
{
    pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64/self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        }else if percentage_of_max >= 0.9 {
            self.messager.send("Urgent warning: Your've used up over 90% of your");
        }else if percentage_of_max >= 0.75 {
            self.messager.send("Warning: You've used up over 75% your quota!");
        }
    }
}


#[cfg(test)] 
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessage {
        sent_messages: RefCell<Vec<String>>,
    }

    impl Message for MockMessage {
        fn send(&self, msg: &str) {
            // 借用可变
            self.sent_messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn it_works() {
        let mock_message = MockMessage {
            sent_messages: RefCell::new(vec![]),
        };
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_message.sent_messages.borrow().len(), 1);
    }
}
