// 测试类型别名
pub fn run(){
    declare_type_alias()

}


// 类型别名来创建类型同义词
//
// 这意味着 Kilometers  是 i32 的同义词, 这里形成的 newtype 不是一个新构建的类型，
// 它只是一个 i32 的别名，被完全当作一个 i32 类型来对待
type Kilometers = i32;
//
fn declare_type_alias() {
    let x: i32 = 5;
    let y: Kilometers = 10;
    // 可以于 i32 数据进行运算
    // 同时，这样就不会得到编译器的类型检查的好处，当我们同时在代码中混用
    // i32 和 Kilometers 也不会让编译器给出一个错误
    println!("x + y = {}", x + y);
}



// 类型别名的主要用途还是减少代码中重复
// 当我们的参数类型是一个很长的类型
fn take_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // --snip --
}

// 将长长的类型参数定义一个别名，
type Thunk = Box<dyn Fn() + Send + 'static>;

fn take_short_type(f: Thunk){
    // --snip--
}


// 类型别名用于在一个 lib 中封装一些大量使用的返回值，使得书写更加简洁
// 例如 在 std::io 模块中 I/O 操作中对 Result<T, E> 中对应的简化, 
// 这里，I/O 操作里面大部分的情况 E 都是 std::io::Error 
// 就可以定义如下的简写方式，于是我们在代码中可以使用 Result<T> 代替后面一大串的内容

type Result<T> = std::result::Result<T, std::io::Error>;


