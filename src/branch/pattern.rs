//
// Rust 中特有的模式，
// -------------------------------------------------------------------
//  Rust 中的模式匹配

pub fn run(){

    match_express();
    if_let();
    while_let();

}

// -------------------------------------------------
// 使用场景
// -------------------------------------------------

// match 模式匹配
fn match_express() -> Option<i32> {

    let x: Option<i32> = Some(29);

    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}


// if let 这样的表达式编译器是无法检查其穷尽了所有的可能，也不要求穷尽所有可能
// 存在处理遗漏的情况编译器也不会警告.
fn if_let() {

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // if let 与 else, else if, else if let 可以组合使用
    // 这与 match 一次只能与一个值与模式相比较提供了更好的灵活性
    // Rust 也对分支的条件不要求相互关联
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the backgroud");
    } else if is_tuesday {
        println!("Tuesday is green day!");

        // 这里引入了一个覆盖变量(与 match 中引入的覆盖变量一样)
        // 如果匹配, 这时 age 包含 Ok 成员中的值. 并且这个值的作用域
        // 在大括号开始时有效，并不在大括号外有效。
    } else if let Ok(age) = age  {
        if age > 30 {
            println!("Using purple as the backgroud color");
        }else {
            println!("Using orange as the backgroud color");
        }
    } else {
        println!("Using blue as the backgroud color");
    }
}


// 只要模式匹配就一直运行 while 循环
fn while_let() {

    // 模拟栈结构，先进后出
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // 每次取出最后一个元素并返回 Some(value) 
    // 当所有的元素斗取完了，返回 None 
    // 循环只要接收到 Some 救会一直执行直到接收到 None
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// for 循环中，模式是for 关键字直接跟随的值，
fn for_expression() {

    let v = vec!['a', 'b', 'c'];

    // 使用 for 循环来解构
    // enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引
    // 他们位于一个元组中(index, value), 解构时自动在对应变量中赋值
    for (index, value ) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

}

