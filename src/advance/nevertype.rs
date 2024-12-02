
// Rust 中有一个 `!` 类型，被称为 never type, 
// 它的作用就是在函数从不返回的时候充当返回值
pub fn run(){}

// 1. 表现形式
//  这里的 let match 这样的分支必须要返回相同的数据类型
fn guess_number() {
    loop {
        let input = "1002";
        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            // 由于 never  type 无法产生一个可供返回的值,
            // 因此这个match 表达式就是返回 Ok 对应的 u32 类型
            // 而 Err 对应的是 never type 被强制转换为 Ok 内类型相同的任何类型
            Err(_) => continue,
        };
        println!("{guess}");
    }
}
