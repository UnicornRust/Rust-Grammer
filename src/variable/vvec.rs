use std::cmp::Ordering;

/// 测试 Vec 数据结构
///
pub fn vec_type() {
    println!("------------------test of vec-------------------------");
    // define vec type
    defind_new_vec();

    // vec macro
    vec_macro();

    // get vet item
    get_vec_item();
    // 获取 vec 的一些元数据
    get_vec_meta();
    // test scope
    test_scope();

    // store with different type in enum
    test_vec_with_enum();
}

fn defind_new_vec() {
    // 定义一个全新的 Vec 类型的变量 v,
    // 我们期待这个 Vec 结构中存储 i32 类型的数据
    // mut 表示可以在里面添加数据
    let mut v: Vec<i32> = Vec::new();
    v.push(100);
    // 当作栈的方式来使用
    v.pop();
}

fn vec_macro() {
    // 通常我们使用 vec 的时候使用的是一系列的初始值
    // 来直接构建一个 vec 容器，这时候会自动根据初始化
    // 时的类型推断出需要存储数据的类型，不需要指定类型
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);
}

fn get_vec_meta() {
    let mut vec2 = vec![1, 2, 3, 4];
    println!("vec size: {}", vec2.len());
    println!("vec capacity: {}", vec2.capacity());
    // vec 数据存储是在堆上的, 也可以通过引用来操作数据
    println!("vec reference: {:?}", &vec2);
    println!("vec reference: {:?}", vec2.as_ptr());
}

// 读取 vec 中值
fn get_vec_item() {
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);

    // 直接使用下标索引的方式来获取项的引用，
    // 这样的方式来获取的时候，当索引不存在的时候会
    // 出现 panic, 程序崩溃
    let _second: &i32 = &vec2[1];

    // get 方式获取的时候, 当索引值不存在的时候会返回 None
    // 这在一些可能会出现索引不存在的场景中很有用，不会导致程序崩溃
    match vec2.get(1) {
        Some(x) => println!("second:{x}:{_second}"),
        None => println!("x is none, {_second}"),
    }

    // 遍历可变引用
    for i in &mut vec2 {
        *i *= 2;
    }

    // 遍历不可变引用
    for i in &vec2 {
        println!("{}", i);
    }

    println!("Vec length: {}", vec2.len());
    // pop 会直接将数据从 vec 中移除
    println!("Pop : {:?}", vec2.pop());
    println!("Vec length: {}", vec2.len());
}

/// vec 的作用范围
/// 当 vec 走出他的作用域的时候将被回收，同时内部的所有元素也将被回收
fn test_scope() {
    // vec 的使用也同样适用于 rust 中的内存借用规则，
    // 一个作用域中不能同时出现可变于不可改变引用。
    //
    let mut v = vec![1, 2, 3, 34, 4, 5];
    // 不可变引用
    let first = &v[2];
    println!("The first number is : {}", first);
    // 可变引用
    v.push(8);
    // 再次使用不可变引用是不被允许的.
    // println!("The first number is : {}", first);
}

/// store different type
/// 当我们需要在 vec 中存储不同的类型的数据的时候，
/// 我们需要借助 enum 来实现，enum 的成员可以包含不同的字段类型
/// 而 enum 的成员被认为是同一种类型，因此可以存储在 vec 中
fn test_vec_with_enum() {
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];
    println!("{:#?}", row);
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
