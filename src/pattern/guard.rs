
// 
// 匹配守卫：是一个指定于 吗通常 分支模式后的额外 if 条件， 
// 它也必须被满足才能选择此分支,匹配守卫用于表达比单独模式所能允许的更为复杂的情况。
//
pub fn run() {
    guard();
    guard_not_cover();
    multi_pattern_guard();

}
fn guard(){
    //
    let num = Some(4);
    match num {
        // 这里的条件可以使用模式中创建的变量
        // 第一个分支还有匹配守卫，if x % 2 == 0 (当偶数时成立)
        // 如果守卫的条件不成立(奇数) 则会继续前往匹配下一个模式
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} ois odd", x),
        None => (),
    }
}


// 无法在模式中表达类似 if x % 2 == 0 这样的逻辑，通过匹配守卫提供
// 这种表达类似逻辑的能力，这种替代表达方式的缺点是，
// 编译器不会尝试为包含匹配守卫模式检查穷尽性

fn guard_not_cover() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // 现在这里不会引入一个覆盖外部 y 的新变量 y, 这意味着
        // 可以在在匹配守卫中使用外部的 y, 相比指定会覆盖外部y
        // 的模式 Some(y), 这里指定为 Some(n), n 并没有覆盖任何值
        // 因为 match 外部没有变量 n
        Some(n) if n == y => println!("Match, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x)
}


fn multi_pattern_guard() {

    let x = 4;
    let y = false;

    match x {
        // 使用 `或` 运算符来匹配多个模式，匹配守卫会作用于所有模式,
        // 语义类似于 (4 | 5 | 6) if y => 
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

}

