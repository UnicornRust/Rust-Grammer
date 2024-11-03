//
// refcell
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
pub fn refcell() {

    let mock_message = MockMessage {
        sent_messages: RefCell::new(vec![]),
    };
    let mut limit_tracker = LimitTracker::new(&mock_message, 100);
    limit_tracker.set_value(80);
    println!("{:?}", mock_message.sent_messages.borrow().len());
}


pub trait Message {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Message> {
    messager: &'a T,
    value: usize,
    max: usize,
}


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
