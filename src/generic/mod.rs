pub mod gentrait;
pub mod gentype;
pub mod lifetimes;



pub fn run() {

    gentype::run();
    gentrait::run();
    lifetimes::run();
}
