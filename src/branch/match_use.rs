use std::cmp::Ordering;
/// 测试 match 模式匹配
pub fn test_match() {
    // test if let
    if_let();

    // test list every variant
    test_dail();

    // test_match_code_block
    test_match_code_block();

    // test match with enum
    test_match_enum();

    // match 的基本使用
    base_match();
}

/// if let 语法糖可以简化只需要处理某种情况的代码
/// 减少模板代码的书写, 当模式与选中的模式匹配，
/// 代码执行，否则不执行，同源可以使用变量绑定max,
/// 这样的写法会丢失穷尽性检查，是代码简洁与严谨的权衡取舍，只适用于特定的场景。
fn if_let() {
    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    // 这样的写法与上述的写法是等价的，为了减少模板样式的代码，
    // 使用下面这样的形式，避免了其他不需要处理情况的代码
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // 可以跟一个 else 分支，但是这样的写法就与原本的模式匹配时同样的
    // 不建议这样写
    else {
        println!("else is no sense");
    }
}

/// match 分支的列举必须是穷尽的，否则不能编译通过
/// 这在 rust 编译器就不能通过编译，编译器甚至会告诉
/// 你哪些情况没有列举出来

fn test_dail() {
    // 假设现在是一个掷骰子的游戏，
    let dice_roll = 9;

    // 针对所有的结果，我们需要列举出来，分别处理
    // 但是这里我们只要对特别的数据(3,7)做处理，
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 当骰子不是 3，7 使用 other 变量作为通配，列举剩下的可能
        // 可以使用 other 变量来获取到对应的值
        // other => all_handle(other)

        // 剩下的值做统一的处理, 与掷的值无关，舍弃这个值
        // _ => reroll(),

        // 当我们不关心剩下的所有的可能，不处理这些值,直接丢弃
        // 使用空元组来表示空逻辑，不做处理
        _ => (),
    }
}

// 加帽子
fn add_fancy_hat() {}
// 摘帽子
fn remove_fancy_hat() {}
// 所有的处理
fn all_handle(other: i8) {}
// 统一处理
fn reroll() {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit");
            }
            Message::Move { x, y } => {
                println!("Move({x},{y})");
            }
            Message::Write(value) => {
                println!("write a value, {}", value);
            }
            Message::ChangeColor(r, g, b) => {
                println!("R:{r},G{g}, B{b}");
            }
        }
    }
}

/// 测试 match 与 enum 一起使用时，
/// enum 可以使用多样化的数据类型
fn test_match_enum() {
    // 枚举的使用
    let write: Message = Message::Write("hello".to_string());
    write.call();
    let quit: Message = Message::Quit;
    quit.call();
    let m: Message = Message::Move { x: 12, y: 24 };
    m.call();
    let color: Message = Message::ChangeColor(255, 255, 255);
    color.call();
}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coins: Coins) -> u8 {
    // match 作为一个表达式，代表了整个函数的返回
    match coins {
        Coins::Penny => {
            // 每一个模式中可以写自己的代码，
            println!("Lucky Penny");
            1
        }
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25,
    }
}

/// 测试 match 分支中可以执行的代码块逻辑
fn test_match_code_block() {
    let mut value = value_in_cents(Coins::Dime);
    println!("{}", value);
    value = value_in_cents(Coins::Nickel);
    println!("{}", value);
    value = value_in_cents(Coins::Penny);
    println!("{}", value);
    value = value_in_cents(Coins::Quarter);
    println!("{}", value);
}

/// match 的比较用法
fn base_match() {
    let my_age = 18;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can't Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };
}
