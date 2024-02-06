// ========================================================
// 模式匹配的语法

pub fn run() {
    match_value();
    match_ags();
}

//
// 匹配字面值
//
fn match_value() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        // 多个模式（ 使用 | 语法匹配多个模式, 代表或运算符)
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 通过 ..= 匹配值的范围(编译器在编译时检查范围不为空)
    // char 和 数字是 Rust 仅有的可以判断范围是否为空的类型，
    // 因此范围只允许用于数字和char 值
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // char 示例
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

//
// 使用 match 完成匹配，
// 并在内部匹配时形成值的覆盖
//
fn match_ags() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("got 50"),
        // 这里匹配的时候，会收任何非 None 的 Option 的值
        // 同时在 match 内部的 y 会被赋值,覆盖外部的y值
        Some(y) => println!("Matched y = {y}"),
        // 这里匹配剩下的所有条件，由于没有新的 x 被定义，还使用外部的 x
        _ => println!("Default case x = {:?}", x),
    }

    // 由于 match 作用域结束，这里荏苒使用的是外部的 x,y
    println!("at the end: x = {:?}, y = {y}", x);
}

// 使用模式来解构结构体, 枚举，元组，以便使用这些值的不同部分
//
//
fn unwrap_struct() {
    let p = Point { x: 0, y: 7 };
    // 模式中的字段名不必与结构体一致，但是为了方便理解，一般是写作一致
    // 这时候就有很多重复的部分，因此可以简写
    let Point { x: a, y: b } = p;
    // let Point { x, y } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    //
    // 也可使用字面值作为结构体模式的一部分进行解构，而不是为所有字段创建变量
    match p {
        // match 表达式一旦匹配一个就会体制检查其他分支，
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x},{y})");
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

//
// 解构枚举
//
fn unwrap_enum() {
    let msg = Message::ChangeColor(19, 160, 225);
    match msg {
        // 没有任何数据的枚举成员不能继续解构其值
        Message::Quit => {
            println!("The Quit variant has no data to destructure");
        }
        // 内含数据的枚举可以在模式匹配的时候对内部数据进行解构并赋值
        // 将匹配的数据的值解构到变量中，方便使用
        Message::Move { x, y } => {
            println!("Move in thr x direction {x} and in the y direction {y}")
        }
        // 包含一个或者多个元素的值类似于解构元组的方式
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("change the color to red {r}, green {g}, blue {b}");
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//
// 解构枚举的嵌套解构，
//
fn unwrap_nest_enum_struct() {
    let msg = Msg::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Msg::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("change color to red {r}, green {g}, blue {b}");
        }
        Msg::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Msg {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

//
// 解构结构体和元组
//
fn unwrap_struct_tuple() {
    // 可以用复杂的方式来混合，匹配和嵌套解构模式，
    // 这里是一个解构体和元组嵌套在另一个元组中,
    // 可以使用解构来分解这些复杂的类型，获取部分我们感兴趣的部分
    let ((feet, inches), Point { x, y }) = ((3, 19), Point { x: 3, y: -10 });
}

