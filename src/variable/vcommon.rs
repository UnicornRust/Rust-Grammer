pub fn variable() {
    variable_declare();
    variable_change();
    variable_shadowing();
}

// 定义的模块中所有内容默认是私有的，
// 需要使用 pub 关键字来声明公开
fn variable_declare() {
    // 变量声明默认是不可变的，需要使用mut关键字来声明可变的变量
    let mut i = 10i32;
    i = i + 1;
    // 变量声明如果不适用会导致编译器出现异常
    println!("i = {}", i);

    // 变量声明是一种模式匹配
    // let i: int
    let (a, b) = (18i32, 20i32);
    println!("a:b = {}:{}", a, b);

    for i in 0..10 {
        println!("i = {}", i);
    }
}

fn variable_change() {
    // mut 修饰的变量才能被重新赋值，否则变量不能被重新赋值，默认是不可变的
    let mut i = 10132;
    i = i + 1;
    println!("i = {}", i);
}

// 变量隐藏:
// ----------------------------------------------
// 使用 let 两次声明同一个变量名时，前面声明的变量将会被隐藏
// 两次可以使用不同的类型，这与 mut 不同，mut 数据类型是不可变
// 这样操作其实是生成了两个变量。前面的变量被隐藏了。
fn variable_shadowing() {
    // 1. 在不同的作用域下，x 被多次声明后不会相互影响
    // 声明了一个不可变的变量 x
    let x = 5;

    // 又声明了一个不可变得变量 x，并赋值 x + 1
    let x = x + 1;

    // 声明一个新作用域，再次声明一个 x 变量，这个变量得作用域就在这个
    // 局部，退出这个作用域，在内部声明的 x 不会影响外部的变量的值
    {
        let x = x + 10;
        println!("inner: {}", x);
    }
    println!("outer: {}", x);

    // 2: 隐藏变量的时候，因为是声明了新的变量，所以变量的类型是可变的
    let space = "  ";
    println!("{}", space);

    let space = space.len();
    println!("{}", space);
}
