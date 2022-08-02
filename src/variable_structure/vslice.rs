/// slice 允许我们引用一个集合中的一段连续的元素序列。而不是一个集合的全部元素。
/// slice 是一类引用，所以它没有所有权

pub fn slice_reference() {
    only_one_reference();

    str_or_string_call_slice();
}

fn only_one_reference() {
    // let mut s = String::from("hello, world");
    let s = String::from("hello, world");
    let slice = first_word(&s);
    // s.clear();
    println!("{slice}")
}

fn str_or_string_call_slice() {
    let s = String::from("hello, world");
    let s1 = "hello";
    let word = one_word(&s);
    let word1 = one_word(s1);
    let word2 = one_word(&s1);
    println!("word: {word}, word1: {word1}, word2:{word2}");
}

// 创建一个函数找到一组字符串的首个单词
fn first_word(s: &String) -> &str {
    // 转化为字节数组
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// 查找字符
// 当我们使用 &str 改写参数之后，这使得我们的程序既可以使用字符串引用调用
// 也可以使用 slice 引用调用
pub fn one_word(s: &str) -> &str {
    // 转化为字符数组
    let chars = s.chars();
    for (i, item) in chars.enumerate() {
        if item == ' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
