use std::ops::Add;

// 演示函数定义的细节
pub fn run() {
    // function_defined
    let result = func_defined(5, 'h');
    println!("{}", result);

    // fibonacci 数列
    print_ten_fibonacci();

    // sum list
    let list = vec![1, 2, 3, 4, 5];
    println!("Sum of list: {}", sum_list(&list));

    println!("5 + 4 = {}", generic_function(5, 4));
    println!("5.4 + 4.5 = {}", generic_function(5.4, 4.5));
}

// 定义参数类型
// 定义返回值类型
fn func_defined(x: i32, c: char) -> &'static str {
    println!("{}{}", x, c);

    let s = "hello";
    // 函数中最后一个表达式被认为是函数的返回值，如果想要提前返回
    // 使用 return 关键字
    // 最后这个表达式 s 相当于 return s;
    // 但是不能加 ; 加上了分号表示一个语句，此时相当于函数没有返回值
    // 默认返回的是 return () 所以编译器报返回值类型不匹配错误
    s
    // return s;
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    };
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn print_ten_fibonacci() {
    for i in 1..=10 {
        let x = fibonacci(i);
        println!("fibonacci:{i} --> {x}")
    }
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for &val in list.iter() {
        sum = sum + &val;
    }
    sum
}

fn generic_function<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}
