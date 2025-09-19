pub mod atomic;
pub mod builder;
pub mod channel;
pub mod mutex;
pub mod simples;
pub mod thread;
pub mod share_data;

pub fn run() {
    // thread::run();
    // channel::run();
    // mutex::run();
    // atomic::run();
    // simples::run_examples();
    // builder::run();
    share_data::share();
}
