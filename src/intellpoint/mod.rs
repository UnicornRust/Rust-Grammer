use std::rc;

use cycle_ref::cycle;

pub mod boxtype;
pub mod rctype;
pub mod refcell;
pub mod rc_refcell;
pub mod cycle_ref;


pub fn run() {

    boxtype::run();
    rctype::run();
    rc_refcell::run();
    cycle_ref::cycle()
}
