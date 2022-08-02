pub fn string_variable() {
    // new string
    new_string();
    // slice stting
    slice_string();
    // iterater string
    iter_string();
    // append string
    append_string();
}

fn new_string() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("new_str: {}", s);
    // 第二种方式
    let _str = "string".to_string();
    // 第三种方式
    let _nstr = String::new();
}

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
    for c in str.bytes() {
        println!("{}-", c)
    }

    // 按照单个字符输出
    let emoje = "🐳,👽,❣️,🌈";
    for b in emoje.chars() {
        println!("{}:", b);
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
    s3 = s3 + "tring";
    println!("{}", s3)
}
