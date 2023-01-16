pub fn tuple_type() {
    tuple_declare();
    tuple_define_every_type();
}

// tuple 类型的声明
fn tuple_declare() {
    let s = (12, 14);
    println!("tuple: {}, {}", s.0, s.1)
}

fn tuple_define_every_type() {
    // 定义一个元组类型, 主要的场景是用于临时存储数据，或者作为一个函数的
    // 返回值，而不用构建一个对象来传递
    // 主要的取值手段就是通过 index
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 1.0000);
    println!("Name: {}", my_tuple.1)
}
