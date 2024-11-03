use std::rc;

pub mod boxtype;
pub mod rctype;
pub mod refcell;
pub mod rc_refcell;


pub fn run() {

    rc_refcell::run();
    boxtype::run();
    rctype::run();
}
