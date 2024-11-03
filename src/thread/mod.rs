pub mod thread;
pub mod channel;
pub mod mutex;

pub fn run() {
    thread::run();
    channel::run();
    mutex::run();
}
