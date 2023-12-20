use std::fs::File;

/// 用于测试泛型参数的应用
pub fn run() {
    println!("-----------------test of generic use ----------");
    // 泛型参数使用
    generic_in_method();
    //
    generic_in_struct();
    //
    generic_in_enum();
    // 结构体方法泛型参数
    generic_in_struct_method();
}

/// 使用泛型参数来使得多种类型都可以使用这个函数
/// 具体使用的类型需要在函数调用的时候传递进来
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // 这里 List[0] 是一拷贝移动操作, 对于哪些存储在栈上的实现了 copy 的类型例如 ： i32, char
    // 这样的操作是没有问题的，但是使用了泛型参数之后，T 的类型可能没有实现 Copy trait, 因此
    // 是不能移动的，要想使用这个操作，必须要限定类型实现 Copy ttait
    let mut larget = list[0];
    for &item in list {
        // 只使用 类型 T 来约束类型，这里依旧不能编译，
        // 因为在方法体中使用了 > 运算符号，因此只用
        // 实现了 stf::cmp::PartialOrd 的 trait 的类型
        // 才能使用这个运算符，因此只有实现这个 trait 的
        // 类型才能调用这个方法， 因此怎样在限定类型的
        // 同时还要限定类型需要实现的 trait
        if item > larget {
            larget = item;
        }
    }
    larget
}

/**
 *  对于上述这样的情况，如果我们想要以其他的一些类型(非copy-trait)也能使用我们的 largest 函数
 *  那么我们可以使用 clone 函数来克隆对应的值，使得largest 函数拥有其所有权，
 *  这样一来在像 String 等类型如果在堆上拥有较多数据，则需要在堆上分配大量空间 相当缓慢
 * ----------------------------------------------------------------------------------
 * 如果我们可以返回 slice 中 T 值的引用，修改返回值，这样一来，
 * 我们就不需要 Clone 或者 Copy 这样的 trait bound.
 */
fn lagest_reference<T: PartialOrd>(list: &[T]) -> &T {
    let mut lagest = &list[0];
    for item in list {
        if item > lagest {
            lagest = item;
        }
    }
    lagest
}

/// generic 在方法中的使用
fn generic_in_method() {
    let number_list = vec![34, 45, 35, 100, 65];
    let result = largest(&number_list);
    println!("The lagest number: {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The lagest char: {result}");
}

/// 结构体使用泛型
/// ---------------------------------------------------------------
/// 结构体名称后使用<> 定义需要使用的泛型参数，多个泛型使用逗号间隔
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn generic_in_struct() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?}:{float:?}");
}

/// 要想在结构体方法中使用泛型，需要在 imlp
/// 后申明这些泛型类型, 这是让 Rust 编译器
/// 明确的知道这是泛型而非具体的类型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/// 构建一个只用于拥有泛型参数 T 的结构体的具体类型的 impl 块
/// 可以为方法添加具体的类型限制，用以表明方法仅仅是
/// 用于当前限制的泛型类型
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2).sqrt()
    }
}
// ------------------------
struct Points<X1, Y1> {
    x: X1,
    y: Y1,
}

/// 结构体中定义泛型参数并不总是于结构体方法签名中使用的泛型是同一类型
impl<X1, Y1> Points<X1, Y1> {
    // 方法中引入了 结构体不同的类型
    fn mixup<X2, Y2>(self, other: Points<X2, Y2>) -> Points<X1, Y2> {
        Points {
            x: self.x,
            y: other.y,
        }
    }
}

// 测试结构体方法于结构体泛型参数不一致的情况
fn generic_in_struct_method() {
    let p1 = Points { x: 5, y: 10.4 };
    let p2 = Points { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

/// 枚举中使用泛型
/// ---------------------------------------------------------------
/// 1.最常使用的 Option<T> 枚举
///enum Option<T> {
///    Some(T),
///    None,
///}
/// 2.非常使用的 Result<T, E> 枚举
/// Result<T, E> {
///     Ok(T),
///     Err(E),
/// }
/// ---------------------------------------------------------------
fn generic_in_enum() {
    let f = File::open("temp/hello.txt").expect("Open file failed");
    println!("{:?}", f);
}
