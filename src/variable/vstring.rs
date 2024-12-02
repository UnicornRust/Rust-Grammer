pub fn string_variable() {
    // new string
    new_string();
    // slice stting
    slice_string();
    // iterater string
    iter_string();
    // append string
    append_string();
    // 与数字之间的转换
    parse_num_str();
    // 获取字符串的指针
    get_ptr();
}

/**
 ---------------------------------------------------------
  不管是 String 类型 还是 slice 字符串都是 UTF-8 编码, Rust
  为字符串提供了多种的不同的方式来解释计算机存储的原始字符串
   - 字节 : 英文单字节，常见的双字节，不常见的三字节
   - 标量值 : unicode 标量值
   - 字形簇 : 由语言所表示的文字或者符号的形式/样式
 ---------------------------------------------------------
*/

fn new_string() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("new_str: {}", s);

    // 第二种方式
    let _str = "string".to_string();

    // 字符串字面量的 into() 方法，凡是调用这个方法无自动推导类型
    // 需要主动标定类型 (实际上是调用 From trait 中的方法，实现了类型转换)
    let _s: String = "hello".into();

    // 第三种方式
    let _nstr = String::new();
}

/**
-----------------------------------------------------------
 String 中没有提供使用索引访问字符串的字符的方式：
  - 因为在 UTF-8 编码的字符串中，字节的索引值与调用者想要获取的字符
    通常不是对应的.
  - 为了避免这样的错误发生, Rust 不让你这样做.
-----------------------------------------------------------
 为了弥补这样的缺失，Rust 中引入了字符串 slice, 允许你划取某一个索引
 范围内的字节从而获得含有特定字符的字符串 slice.
*/
fn slice_string() {
    let s = String::from("hello world");
    // 截取某一个字节范围的字符串
    // 由于字符串是 UTF-8 编码的，因此截取的字符串也是 UTF-8 编码的
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("slice: {}-{}", hello, world);

    // 需要直到知道具体某一个字符是多长，才不会出现编译错误
    // 当出现宽字节，而截取的位置不对会出现错误

    let emoje = "🐳,👽,❣️,🌈";
    let demoje = &emoje[0..4];
    let eemoje = &emoje[5..9];
    let hemoje = &emoje[10..13];
    let lemoje = &emoje[17..21];
    println!("{}:{}:{}:{}", demoje, eemoje, hemoje, lemoje)
}

fn iter_string() {
    // 按照字节输出
    let str = "hello cargo";
    for b in str.bytes() {
        println!("{}-", b)
    }

    // 按照单个字符输出
    let emoje = "🐳,👽,❣️,🌈";
    for c in emoje.chars() {
        println!("{}:", c);
    }

    //
    let s = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = s.chars().collect();
    // 排序
    v1.sort();
    // 去重
    v1.dedup();
    for c in v1 {
        println!("{}, ", c)
    }
}

fn append_string() {
    let mut s1 = String::from("hello");
    let mut s2 = String::new();
    let mut s3 = "s".to_owned();
    // push a char,
    s1.push('!');
    println!("{}", s1);

    // 第二种方式, 直接追加字符串
    s2.push_str("world~");
    println!("{}", s2);

    // 第三种方式，`+` 运算符
    // `+` 运算会调用一个内部 add(self, other: &str) -> String {} 函数
    // 这里会移动左侧的变量，同时右侧变量需要是 &str 类型
    s3 = s3 + "tring";
    println!("{}", s3);

    // format!() 会返回一个 String, 不会获取任何参数的所有权
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("new_str: {}", s);
}

fn parse_num_str() {
    const ONE_MIL: u32 = 1_000_000;
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + ONE_MIL;
    println!("I'm {} and I want ${}", age, ONE_MIL)
}


fn get_ptr() {
    let s = String::from("work later");
    let p : *const u8 = s.as_ptr();
    println!("p: {:?}", p);
}

