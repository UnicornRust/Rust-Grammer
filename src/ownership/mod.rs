pub mod fixsize;
pub mod dynamicsize;
pub mod shareReference;
pub mod ownership;


///! 所有权规则
///
/// 1. 
/// 2. 
/// 3.

pub fn run() {
    ownership::run();
    fixsize::fix_size_memory();
    fixsize::fix_size_own();
    dynamicsize::dyn_size_ptr();
    dynamicsize::dyn_own();
    shareReference::exclude();
    shareReference::share_container();
}
