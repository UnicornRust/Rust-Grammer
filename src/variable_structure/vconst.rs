pub fn const_variable() {
    declare_const();
}

// 常量的定义：
// 1. 常量可以在任何作用域中声明

fn declare_const() {
    // 1. 常量声明的时候，需要注明类型
    // 2. 常量需要是编译时计算的常量值，不能是运行时计算的变量值
    // 3. 常量名：约定的命名规则是 ：字母大写，下划线连接
    // 4. 在常量声明的作用域中，整个生命周期都是有效的
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{}", THREE_HOURS_IN_SECONDS);
}
