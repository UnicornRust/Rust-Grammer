pub fn loop_grammer() {
    loop_loop();
    loop_for();
    loop_while();
    loop_break();
    loop_dot_dot();
}
fn loop_loop() {
    let arr = [1, 2, 3, 4, 5];
    let mut x = 0;
    // loop 中的语句至少执行一次, 在判断条件之前执行
    loop {
        if x == 5 {
            break;
        }
        println!("loop: {}", arr[x]);
        x += 1;
    }
}

fn loop_while() {
    let arr = [1, 2, 3, 4, 5];
    let mut x = 0;
    while x < 5 {
        println!("while: {}", arr[x]);
        x += 1;
    }
}

fn loop_for() {
    let arr = [1, 2, 3, 4, 5];
    for x in arr {
        println!("for: {}", x);
    }
}

fn loop_break() {
    let arr = [90, 80, 70, 60, 50, 5, 3, 1];
    let mut x = arr.len() - 1;
    let ok = loop {
        if x < 5 {
            // break 可以在循环中使用, 跳出循环的时候可以返回一个值，
            // 并可以在表达式外部接收到这个值
            break x + 1;
        }
        println!("{}", arr[x]);
        x = x - 1;
    };
    println!("break return {}", ok);
}

// 使用 .. 表示一个序列
// 0..5  --> [0,5)
// 0..=5 --> [0,5]
fn loop_dot_dot() {
    let arr = [10, 20, 30, 40, 50, 60];

    // 含头不含尾
    for i in 0..5 {
        println!("dot: {}", arr[i]);
    }

    // 包含尾部
    for i in 0..=5 {
        if i == 5 {
            println!("index 5 : {}", arr[i]);
        }
    }

    // 反转序列
    for i in (0..=5).rev() {
        println!("rev: {}", arr[i]);
    }
}
