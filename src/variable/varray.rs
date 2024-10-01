use core::time;

// rust 中的数组
pub fn run() {
    declare()
}

/**
* 数组越界访问在编译器不会报错(简单的场景会报错，但是如果是一些动态的数据，则无法在编译器检查出越界错误)
* 但是在运行期会报错
*/

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

