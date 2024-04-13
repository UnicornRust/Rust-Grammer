pub fn run() {
    ignore_with_lowdash();
    nest_lowdash();
    ignore_var();
    ignore_some();
    ignore_flexiable_with_double_dot();
}

//
// 忽略模式中的值
// -----------------------------------------------------------------
// 比如 match 中最后捕获全部情况的分支实际上没有做任何事,但是它确实
// 对所有剩余情况负责，有一些简单的方法可以忽略模式中全部或者部分值
// 使用 _ 模式, 在另一个模式中使用_模式，使用一个下划线开始的名称。
// 或者使用.. 忽略所剩部分的值。
//
fn ignore_with_lowdash() {
    foo(10, 20);
}

// 使用 _ 来忽略整个值
//
fn foo(_: i32, y: i32) {
    println!("This code only use the y parameter: {}", y);
}

// 使用嵌套的 _ 忽略的部分值
//
fn nest_lowdash() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't override an existing customized value");
        }
        // 这里当 setting_value, new_setting_value 任一为None),
        // 会进入这个分支
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
}

//
fn ignore_some() {
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        // 忽略元组中的第二个和第四个值
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
}

//
// 通过在变量名称前添加_ 来忽略对应的变量, 但是这样做只是编译器不会出现警告信息
// 但是值依然会绑定到变量，而只有 _ 的时候不会绑定任何值
//
fn ignore_var() {
    let _x = 5;
    let y = 10;
    println!("{y}");

    let s = Some(String::from("hello!"));

    // 此时s 被移动到 _s, 并阻止再次使用s
    if let Some(_s) = s {
        println!("found a string");
    }

    //  s 被部分移动，不再允许使用
    // println!("{:?}", s);

    // 如果我们使用 Some(_) 来完成匹配的时候并没有移动s
    // 后面可以继续使用 s
    // if let Some(_) = s {
    //     println!("found a String");
    // }
    // println!("{:?}", s);
}

// 用 .. 忽略部分值
// 对于有多个部分的值，可以使用 ..
// 语法来使用特定部分并忽略其他值，同时避免不得不每一个忽略值列出
// .. 模式会忽略模式总共剩余的任何没有显式匹配的值的部分。
fn ignore_double_dot() {
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        // 我们只需要 x 的坐标，忽略 y, z 坐标
        // 特别适合有很多字段的结构体但是只涉及到一两个字段被使用的时候
        Point { x, .. } => println!("x is {}", x),
    }
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

// .. 会自动扩展为所需要的值的数量
fn ignore_flexiable_with_double_dot() {
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        // 自动解构，剩下第一个和最后一个，中间是动态大小
        // 这里 .. 使用时必须是无歧义的，如果期望的值和忽略的值是不明确的，
        // Rust 编译器会给出一个错误 例如(.., second,..) 就无法具体知道需要解构的值的位置
        (first, .., last) => {
            println!("some numbers: {first}, {last}");
        }
    }
}
