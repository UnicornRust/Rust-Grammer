pub mod atomic;
pub mod builder;
pub mod channel;
pub mod mutex;
pub mod thread;
pub mod share_data;
pub mod samples;
pub mod rwlock;
pub mod oncelock;
pub mod oncell;

pub fn run() {
    // thread::run();
    // channel::run();
    // mutex::run();
    // atomic::run();
    // builder::run();
    // share_data::share();
    // samples::run_examples();
    // rwlock::run();
    // oncell::run();
    oncelock::run();
}
