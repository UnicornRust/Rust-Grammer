pub mod panic_macro;
pub mod result;
pub mod scene;

pub fn run() {
    panic_macro::run();
    result::run();
    scene::run();
}
