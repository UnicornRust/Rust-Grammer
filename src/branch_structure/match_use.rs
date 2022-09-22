/// 测试 match 模式匹配
pub fn test_match() {
    //
    let value = value_in_cents(Coins::Dime);
    println!("{value}");
}

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
                println!("write a value");
            }
            Message::ChangeColor(r, g, b) => {
                println!("R:{r},G{g}, B{b}");
            }
        }
    }
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
            return 1;
        }
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25,
    }
}
