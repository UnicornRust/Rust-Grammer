#![allow(dead_code)]

use crossbeam::channel::{unbounded, Receiver, Sender};
use log::{info, LevelFilter};
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, thread};
use std::{io, str::FromStr};
use strum_macros::{Display, EnumString};

use crate::eventloop::handler::{DBTestEventHandler, TestEventHandler};

//
// 测试事件循环
//
pub fn run() {
    // env_logger 实现对日志打印级别的过滤输出
    env_logger::builder().filter_level(LevelFilter::Info).init();

    // 获取事件处理器
    let mut event_loop = Dispatcher::new();

    // 注册事件
    event_loop.register_handler(Event::Dummy, Arc::new(TestEventHandler));
    event_loop.register_handler(Event::ExampleEvent, Arc::new(DBTestEventHandler));

    // 开启事件处理线程，迭代遍历事件队列中的所有任务
    event_loop.start();

    // 主线程阻塞，事件由用户输入并触发
    loop {
        // 日志记录的宏组件, 需要 (log, env_log) crate 的支持
        info!("Give me some input, type exit to quit");

        let mut input = String::new();

        // 读取用户输入的内容
        io::stdin()
            .read_line(&mut input)
            .expect("Error during input");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let mut split = input.split_whitespace();
        let name_data = (
            split.next().unwrap_or_default().to_string(),
            split.next().unwrap_or_default().to_string(),
        );

        let event = Event::from_str(&name_data.0).unwrap_or_else(|_| Event::Dummy);
        event_loop.trigger_event(event, name_data.1.as_bytes().to_vec());
    }
}

// 支持的事件枚举类型
#[derive(Clone, Debug, PartialEq, Eq, Hash, Display, EnumString)]
pub enum Event {
    Dummy,
    ExampleEvent,
}

// 事件绑定的数据
pub type Payload = Vec<u8>;

// 事件需要实现的 trait, 实现另外两个 trait 的目的是为了
// 保证在多线程下的并发，和数据在不同的线程中传递的安全性
pub trait Handler: Send + Sync {
    // 事件的处理函数
    fn handle_event(&self, event: Event, payload: Payload);
}

// 事件监听者
#[derive(Clone)]
pub struct Listener {
    pub event: Event,
    pub handler: Arc<dyn Handler>,
}

// 前置处理器, 包含了发送和接收数据的事件通道
// 包含一个数据安全的并发集合保存了一个列表的 handler
pub struct Dispatcher {
    tx: Sender<(Event, Payload)>,
    rx: Receiver<(Event, Payload)>,
    registry: Arc<Mutex<HashMap<Event, Vec<Arc<dyn Handler>>>>>,
}

//
// 为 Dispatcher 指定处理函数
//
impl Dispatcher {
    // 构建 Dispatcher 对象
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Dispatcher {
            tx,
            rx,
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    // 注册事件处理器
    pub fn register_handler(&mut self, event: Event, handler: Arc<dyn Handler>) {
        let mut registry = self.registry.lock().unwrap();
        registry.entry(event).or_insert_with(Vec::new).push(handler)
    }
    // 触发事件
    pub fn trigger_event(&self, event: Event, payload: Payload) {
        // 即将事件发送到信道 channel 中
        self.tx.send((event, payload)).unwrap();
    }
    // 监听 channel 中收到的数据, 并开启新的线程来处理
    pub fn start(&self) {
        let registry = Arc::clone(&self.registry);
        let rx = self.rx.clone();
        thread::spawn(move || loop {
            if let Ok((event, payload)) = rx.recv() {
                let registry = registry.lock().unwrap();
                if let Some(handlers) = registry.get(&event) {
                    for handler in handlers {
                        handler.handle_event(event.clone(), payload.clone());
                    }
                }
            }
        });
    }
}
