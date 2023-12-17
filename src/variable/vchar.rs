// 字符类型的变量的操作
pub fn char_variable() {
    char_declare();
}

fn char_declare() {
    // 字符型变量使用的是 单引号包裹，
    // 大小为 四个字节，虽然 一个 Unicode 字符也是四个字节
    // 但是并不能意味着 一个 char 表示一个 Unicode 标量
    // Rust 中由于使用了四个字节，因此比 ASCII 表示更多的内容
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("char : {} - {} - {}", c, z, heart_eyed_cat);
}
