pub mod thread;
pub mod channel;
pub mod mutex;
pub mod atomic;
pub mod simples;

pub fn run() {
    // thread::run();
    // channel::run();
    // mutex::run();
    // atomic::run();
    simples::run_examples();
}
