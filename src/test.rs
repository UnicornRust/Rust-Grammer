
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
///  2,3 : 打印参数时使用了 debug 格式，因此要求参数实现了 PartialEq 和 Debug trait ()
///
#[test]
fn test_assert() {
    assert!(1 + 1 == 2);
    assert!(1 + 2 == 4, "not equals {}", "1 + 2 == 4" )
}

#[test]
fn test_assert_eq() {
    assert_eq!(1 + 1, 2);
}

#[test]
fn test_assert_nq() {
    assert_ne!(1 + 3, 2);
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



/// 将 Result<T, E> 作为返回类型, 表示测试的结果
/// 1. 程序无须发生 panic
///  > 返回 `Ok` 测试通过
///  > 返回 `Err` 测试失败
/// 2. 不要在使用 Result<T, E> 作为测试结果的测试用例上标注#[should_panic]

#[test]
fn test_result() -> Result<(), String> {
    println!("test result used in test case");
    Ok(())
}


/// -----------------------------------------
/// 测试运行的控制指令
/// -----------------------------------------
/// 1. 默认测试是并发运行的，可以通过参数控制测试运行的线程数量
///    > `cargo test -- --test-threads=1`
/// 2. 可以通过对测试标记 #[ignore]，忽略该测试(耗时很长的测试)
///    > `cargo test -- --ignored`  // 可以运行被标记为忽略的测试
/// 3. 可以指定测试的范围, (模块, 前缀，文件，函数等)
///    > `cargo test {test_name(only)/partical_name(batch)/test_module_name(module)}` 
///    > ``
/// 4. 控制测试的输出, 一般情况下，测试通过的时候不会输出任何信息，
///    只有测试失败的时候才会显示标准输出(stdout)
///    > `cargo test -- --nocapture`  // 不拦截程序的输出
#[test]
#[ignore]
fn test_ignore(){

}
