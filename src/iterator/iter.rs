///
/// 用于测试演示迭代器的使用
/// 在 Rust 中，迭代器是惰性的，这意味着在调用方法使用迭代器之前都不会有这个效果
///
pub fn run() {
    println!("---------------test of iter -----------------");
    // 迭代器的创建
    iter_create();
    // 手动 迭代
    iter_demonstration();
    // 消费适配器
    iter_consuming_adaptors();
    // 迭代适配器
    iter_iterator_adaptor();
    // 捕获环境的闭包作为迭代器的参数
    iter_catch_env_closure();
}

//
// 1. iter 的声明使用
//
fn iter_create() {
    //
    let v1 = vec![1, 2, 3];
    // 这里迭代器被声明后存储在变量 v1_iter 中，一旦创建后，可以选择用多种方式利用它
    // 我们使用 for 循环来遍历一个数组并在每一项上执行一些代码，在底层它隐式地创建并
    // 接着消费了一个迭代器。
    let v1_iter = v1.iter();

    // 当我们使用 for 循环来迭代的时候，比不要求迭代器是可变的
    // 这是因为 for 循环获取了迭代器的所有权，在内部改变了迭代器
    for val in v1_iter {
        println!("Got: {}", val)
    }
}

/// 2.迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait, 这个 trait 中定义了
///  - 一个 tarit 的关联类型，这个类型作为 next 返回值的类型，就是迭代器返回元素类型
///  - 一个 next() 方法, 这是迭代器实现着被要求定义的唯一方法，next 一次返回迭代器
///     中的一个项, 封装在 Some 中，当迭代器结束时则返回 None. 可以通过调用 迭代器的
///     next() 方法来获得迭代器的下一个元素。
/// -------------------------------------------------------------------------------
///  迭代器的种类：
///   - `iter()` : 获取到的是原数据的不可变引用, 不可变应用的迭代器
///   - `into_iter()` : 使用这个方法将发挥拥有所有权的迭代器。
///   - `iter_mut()` : 可获得迭代可变引用的迭代器
fn iter_demonstration() {
    let v1 = vec![1, 2, 3];
    // 需要注意的是：当需要调用 迭代器 next() 方法时，next() 会改变了迭代器中用来记录
    // 序列位置的状态. 所以 v1_iter 需要是可变的。
    let mut v1_iter = v1.iter();
    println!("{}", v1_iter.next().unwrap_or_else(|| &0));
    println!("{}", v1_iter.next().unwrap_or_else(|| &0));
    println!("{}", v1_iter.next().unwrap_or_else(|| &0));
    println!("{}", v1_iter.next().unwrap_or_else(|| &0));
}

///
/// 3. 迭代器的方法
/// ------------------------------
/// Iterator trait 有一系列不同的由标准库提供默认实现的方法，其中一些定义中调用了
/// next() 方法，这也是为什么在实现 Iterator trait 的时候要求实现 next() 方法
/// ----------------------
/// > 这些调用了 next() 方法的方法被称为 `消费适配器`，因为他们调用的时候会消耗迭代器
fn iter_consuming_adaptors() {
    iter_sum();
}

// 当器遍历每一个项时，并进行累加计数。并在迭代完成时返回总和,
// 调用 sum 之后不再允许使用迭代器，因为调用 sum() 时它会回去哦迭代器的所有权
//
fn iter_sum() {
    let v1 = vec![1, 3, 5];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("{}", total);
}

/// > Iterator trait 中海定义了另一类方法，被称为 `迭代器适配器(Iterator adaptors)` ,
///   他们允许我们将当前迭代器变为不同类型的迭代器，可以链式调用多个迭代器适配器，不过因为
///   所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果.
///   -----------------------
///   可以链式调用多个迭代器适配器来进行复杂的操作
fn iter_iterator_adaptor() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // 因为所有的迭代器适配器都是惰性的，因此我们单独使用他们(例如这里使用 map)
    // 的时候并不会执行任何操作，总是需要一个消费适配器来获取迭代适配器的结果
    // 这里使用 collect() 来收集结果到一个 vector 中，他们是源数据的项加一后得到的 vector
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{:#?}", v2)
}

///  
/// 4. 使用捕获其环境的闭包
/// -----------------------------------
/// 很多迭代器接受闭包作为其参数，而通常指定为迭代器适配器参数的闭包会是捕获其环境的闭包
/// 下面是 filter 方法接受获取一个闭包，改闭包从迭代器中获取一项并返回一个bool, 如果返回
///  - true : 其值将会被保存在 filter 提供的新的迭代器中。
///  - false : 其值不会被包含在新的迭代器中
///
fn iter_catch_env_closure() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_size(shoes, 10);

    println!("{:#?}", in_my_size);
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

/// 函数中调用了 into_iter 来创建一个获取 vector 所有权的迭代器，接着调用 filter
/// 将这个迭代器适配成一个只含有那些闭包返回 true 的元素的新迭代器.
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 闭包从环境中捕获了 shoe_size 变量，并将每一只鞋的大小做比较，只保留指定大小
    // 的鞋子，最终，调用 collect 将迭代器返回的值收集进一个 vector 并返回
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
