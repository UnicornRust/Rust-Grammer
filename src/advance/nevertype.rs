
// Rust 中有一个 `!` 类型，被称为 never type, 
// 它的作用就是在函数从不返回的时候充当返回值
pub fn run(){


}


// 1. 表现形式
//  这里的 let match 这样的分支必须要返回相同的数据类型
fn guess_number() {
    loop {
        let input = "1002";
        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}
