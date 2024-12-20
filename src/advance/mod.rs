pub mod raw;
pub mod union;
pub mod abi;
pub mod global;
pub mod newtype;
pub mod alias;
pub mod nevertype;
pub mod usestatic;

pub fn run() {
    raw::run();
    union::run();
    newtype::run();
}
