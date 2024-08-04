
fn run() {
    // 4. 外部程序调用 FFi
    // 所有的外部函数的调用被认为是不安全的, 需要使用 unsafe 包裹 
    unsafe {
        println!("absolute value of -3 according to C : {}", abs(-3))
    }
}

// 在当前的代码块中，列出了我们希望能够调用的另一个语言中
// 的外部函数的签名和名称, "C" 部分定义了外部函数所使用的
// `应用额二进制接口(application binary interface ABI)`
// ABI 定义了如何在汇编语言层面调用此函数
// "C" ABI 是最常见的，遵循 C 编程语言的 ABI
//
extern "C" {
    fn abs(input: i32) -> i32;
}
