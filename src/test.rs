
#[cfg(test)]

/// -------------------------------------------------------------
/// 在 test 中的 3A 原则
///  - Arange : 数据准备, 
///  - Action : 代码执行
///  - Assert : 结果断言


///
/// #[test]  Attribute 中标记的函数被认为是一个测试, 
///  - 每一个测试在一个线程中执行，在没有预期 panic 的时候，发生 panic 被认为是测试失败
///
#[test]
fn test_use() {
    panic!("test panic")
}

///
/// 常用的 断言宏
///  1. assert!() , 接收一个 bool 类型的参数，如果 true 测试通过，false 自动 panic, 则测试失败
///  2. assert_eq!(), 接收两个表达式，如果两个表达式运算之后的结果相等，则通过，反之panic;
///  3. assert_nq!(), 接收两个表达式，如果两个表达式运算之后的结果不等，则通过，反之panic;
///
///  1,2,3 : 都可以在末尾添加自定义的测试信息，该测试信息会被传入 format!()
///  2,3 : 测试失败的时候会分别显示左表达式的值，和右表达式的值，方便我们判断测试的过程
///
#[test]
fn test_assert() {
    assert!(1 + 1 == 2);
}

/// 
/// 测试应该发生的 panic, 使用 #[should_panic] 
/// #[should_panic] : 
/// - 预期会发生 panic, 可以通过expected 参数指定预期panic 发生的错误信息
/// - 没有发生预期的 panic(没有发生 panic / 不是预期的 panic)，测试则失败
///

#[test]
#[should_panic(expected = "test panic")]
fn test_should_panic() {
    panic!("test panic")
}


/// -----------------------------------------
/// 测试运行的控制指令
/// -----------------------------------------
/// 1. 默认测试是并发运行的，可以通过 --test-threads=1 来改成单线程运行
/// 2. 可以通过标记为 #[ignore] 的测试，忽略该测试这样的测试在普通的测试集合中会被忽略，
///    如果需要运行忽略的测试，可以通过 cargo test -- --ignored 来运行     
/// 3. 

#[test]
#[ignore]
fn test_ignore(){}
