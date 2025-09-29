pub mod option;
pub mod result;
pub mod drop;


// 对外公开的测试测试方法
pub fn run() {
   drop::drop_occation(); 
   option::run();
}
