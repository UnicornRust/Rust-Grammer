// 数字类型的变量的使用
pub fn number_variable() {
    number_categary();
    number_atithemetic();
}

// 数字类型的种类
fn number_categary() {
    // 1. 整型, 两种数字的范围如下
    //  > 有符号 : -2^(n-1) ~ 2^(n-1) -1
    //  > 无符号 : 2^(n) -1
    //  > isize / usize 类型大小取决于运行的计算平台
    let i: i32 = 9090;
    println!("{}", i);

    // 数字字面量表示的时候可以使用以下任意一种进制
    // (Decimal : 十进制) 98_000
    // (Hex : 十六进制)   0xff
    // (Octal : 八进制)   0o77
    // (Binary : 二进制)  0b11101
    // (Byte : 单字节u8)  b'A'
    let hex = 0xff;
    println!("Hex: {}", hex);

    // 当变量的值大于本身的范围的时候，会出现整形溢出的错误
    // (integer overflow)
    //  > 在 debug 模式下编译，编译器会监测可能出现的溢出问题
    //  > 但是在 release 模式下，不检测溢出，会进行二进制补码包装
    //  > (two's complement wrapping) 操作，例如 256 变成 0，257 变成 1

    // --------------------------------------------------------------------
    // 2. 浮点型 (floating-point number)
    //    > 所有的浮点型都是有符号的。
    // f32 :
    // f64 :

    // 默认的浮点类型是 f64，显式声明的 f32 才会被使用
    let x = 2.0; // f64
    println!("float number : {}", x);

    let y: f32 = 3.0;
    println!("type of y: {}", (y))
}

// 数值计算
// ---------------------------------------------------------------
//
fn number_atithemetic() {
    // addition
    let sum = 5 + 10;
    println!("5 + 10  = {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference);

    // multiple
    // 不同类型的数字不能相互之间运算
    // let production = 4 * 39.5;
    let production = 4 * 395;
    println!("4 * 395 = {}", production);

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {}", quotient);

    let floored = 2 / 3; // result : 0
    println!("floored:  2 / 3 = {}", floored);
}
