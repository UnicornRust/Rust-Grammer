//
// Rust 中特有的模式，
// -------------------------------------------------------------------
// Rust 中的模式匹配

pub fn run() {
    match_express();
    if_let();
    while_let();
    for_expression();
    let_expression();
    as_args();
}

// -------------------------------------------------
// 使用场景
// -------------------------------------------------

// match 模式匹配
fn match_express() -> Option<i32> {
    let x: Option<i32> = Some(29);

    match x {
        None => None,
        Some(i) => Some(i + 1),
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
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");

        // 这里引入了一个覆盖变量(与 match 中引入的覆盖变量一样)
        // 如果匹配, 这时 age 包含 Ok 成员中的值. 并且这个值的作用域
        // 在大括号开始时有效，并不在大括号外有效。
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
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
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

// let 语句
fn let_expression() {
    // 你可能在使用下面这样的语句的时候并没有这样的感觉
    // 每一次这样使用的时候就是在使用模式(我们理解一个赋值就是一个特别简单的模式)
    // 更为通用的语法是 let PATTERN = EXPRESSION;
    // 在赋值的时候将 EXPRESSION 与 PATTERN 比较。将匹配到的值绑定到变量x
    let x = 5;

    // 使用赋值的方式来解构一个元组, 发现值与模式匹配则分别进行赋值
    // 当模式中元素的数量不匹配元组中元素的数量，则整个类型不匹配，编译错误
    let (x, y, z) = (1, 2, 3);
    println!("{x}, {y}, {z}");

    // 可以使用_, .. 来忽略模式中的值
    // let (a, b, ..) = (4, 5, 6);
}

// 作为函数的参数
fn as_args() {
    // 函数参数也可以是模式，
    let point = (1, 4);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
