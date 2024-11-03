pub mod matching;
pub mod atoperation;
pub mod pgrammer;
pub mod ignore;
pub mod guard;


pub fn run() {
    matching::run();
    atoperation::run();
    pgrammer::run();
    ignore::run();
    guard::run();
}
