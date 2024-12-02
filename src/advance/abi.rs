
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


// 其他语言调用 rust 函数
// 这里是不需要 unsafe 的, 除非使用了不安全的 rust 
//
// 1.可以使用 extern 创建接口，其他语言通过他们可以调用 Rust 函数
// 2.fn 关键字前指定 extern 关键字，并指定 "ABi"
// 3.#[no_mangle] 注解主要是防止代码编译优化时被重命名
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("called from C");
}
