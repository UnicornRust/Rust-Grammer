pub fn branch_grammer() {
    if_use();
    if_return();
}

fn if_use() {
    let x = 5;
    if x == 5 {
        println!("x is 5");
    } else if x == 6 {
        println!("x is 6");
    } else {
        println!("x is not 5 or 6");
    }
}

// 可以将 if 代码块看作表达式，可以返回值
// 不支持三元表达式的语法
fn if_return() {
    let condition = true;
    // if 与 else 表达式中返回的值必须是一致的
    let res = if condition { 12 } else { 30 };
    // 不支持
    // let ok = condition ? 12 : 30;
    println!("condition return : {}", res);
}
