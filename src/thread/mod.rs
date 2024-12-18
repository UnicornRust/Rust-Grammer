pub mod thread;
pub mod channel;
pub mod mutex;
pub mod atomic;

pub fn run() {
    thread::run();
    channel::run();
    mutex::run();
}
