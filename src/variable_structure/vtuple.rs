pub fn tuple_type() {
    tuple_declare();
}

// tuple 类型的声明
fn tuple_declare() {
    let s = (12, 14);
    println!("tuple: {}, {}", s.0, s.1)
}
