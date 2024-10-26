
pub fn run() {


}

// 1. 预导入模块 core::options::Option<T>

fn declare() {
    // 使用 some 来定义一个变量会自动推断出 Option<T> 中的 T 类型
    //
    let hello = Some("hello");
    let number = Some(5);
    // 使用 None 来定义类型的时候，无法推断出类型, 因此需要指定类型
    let none: Option<i32> = None;


    // Option<T> 与 T 是不同的类型
    let x = 5;
    let y = Some(10);

    // 必须要类型相同
    let result = x + y.unwrap();
    
    println!("{result}");
}

