use std::{sync::{atomic::{AtomicU32, Ordering}, OnceLock}, thread};


///! OnceLock<T> 是 OnceCell<T> 的线程安全版本
///!  > 一种同步原语，只能被写入一次，
///!  > 是线程安全的 OnceCell<T>, 可以在 static 中使用 
///!  > 用于线程安全的，一次性的变量初始化，并且可以根据调用者使用不同的初始化函数  
///

pub fn run() {
    api();
    deque(); 
}


static LOCK: OnceLock<usize> = OnceLock::new();

fn api() {

    // 初始化状态
    assert!(LOCK.get().is_none());

    // 获取并初始化
    thread::spawn(|| {

        let value = LOCK.get_or_init(|| 12345);
        assert_eq!(value, &12345);

    }).join().unwrap();

    // 再次获取
    assert_eq!(LOCK.get(), Some(&12345));
}


#[derive(Debug)]
struct OnceList<T> {
    data: OnceLock<T>,
    next: OnceLock<Box<OnceList<T>>>,
}

impl<T> OnceList<T> {

    const fn new() -> OnceList<T> {
        OnceList {
            data: OnceLock::new(),
            next: OnceLock::new(),
        }
    }

    fn push(&self, value: T) {
        // 如果设置失败则尝试 next
        if let Err(value) = self.data.set(value) {
            let next = self.next.get_or_init(|| Box::new(OnceList::new()));
            next.push(value);
        }
    }

    fn contains(&self, value: &T) -> bool 
        where T: PartialEq
    {
        self.data
            .get()
            .map(|item| item == value)
            .filter(|v| *v)
            .unwrap_or_else(||{
                self.next
                    .get()
                    .map(|next| next.contains(value))
                    .unwrap_or(false)
            })
    }
}


static LIST: OnceList<u32> = OnceList::new();
static COUNTER: AtomicU32 = AtomicU32::new(0);

const LEN: u32 = 10;

fn deque() {
    thread::scope(|s|{
        for _ in 0..thread::available_parallelism().unwrap().get() {
            s.spawn(|| {
                while let i @ 0..LEN = COUNTER.fetch_add(1, Ordering::Relaxed){
                    LIST.push(i);
                }
            });
        }
    });

    println!("{:#?}", LIST);
    for i in 0..LEN {
        assert!(LIST.contains(&i));
    }
}
