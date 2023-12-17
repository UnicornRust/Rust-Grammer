// 布尔类型的声明与使用
pub fn bool_variable() {
    bool_declare()
}

fn bool_declare() {
    // 布尔类型的值只有两种

    let t = true;

    let f: bool = false;

    println!("{} != {}", t, f)
}
