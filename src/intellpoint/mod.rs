
pub mod boxtype;
pub mod rctype;
pub mod refcell;
pub mod rc_refcell;
pub mod cycle_ref;
pub mod cell;

mod phantomData;


pub fn run() {

    boxtype::run();
    rctype::run();
    rc_refcell::run();
    cell::cell_run();
    cycle_ref::cycle()
}
