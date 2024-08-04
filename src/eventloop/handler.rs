use log::{info};
use crate::eventloop::play::{Handler, Event, Payload};

pub struct TestEventHandler;

impl Handler for TestEventHandler {

    fn handle_event(&self, event: Event, payload: Payload) {
        let data = String::from_utf8(payload).unwrap();
        let message = format!("{} => {}", event, data);
        info!("TestEvent: {}", message);
    }
}


pub struct DBTestEventHandler;

impl Handler for DBTestEventHandler {
    fn handle_event(&self, event: Event, payload: Payload) {
        let data = String::from_utf8(payload).unwrap();
        let message = format!("{} => {}", event, data);
        info!("DBEvent: {}", "Data saved on DB");
    }
}
