// rust 中的数组
fn run() {}

fn declare() {
    // 声明一个数组，
    // 数组是一块已知固定大小的连续内存数据结构，
    // 可以在栈上分配
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "Auguest",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{}", months[0]);
    println!("{}", months[2]);

    // 索引溢出, 程序异常会立即退出
    // println!("{}", months[12]);

    // 数组的索引从0开始
    // 初始化所有的值为 3
    let a: [i32; 5] = [3; 5];
    // let a: [i32, 5] = [3, 3, 3, 3, 3]
    println!("{:#?}", a);
}
