// test for ownership
pub fn run() {
    simple_copy_trait();

    no_copy_trait();

    // 需要创建可变引用，需要变量可变，同时创建引用的时候创建可以引用
    let mut data = String::from("The value world is True");
    calculate_len(&data);

    // 不同作用域的多个可变引用
    more_mut_reference();

   
    // 不能同时存在不可变引用和可变引用
    cant_mut_with_imut();

}

/**
*  copy trait: 可以像整数一样完全存放在 stack 上的类型
*  ----------
*  1. 如果一个类型实现了 Copy 这个 trait, 那么旧的变量在赋值之后依旧可用
*  2. 如果一个类型或者该类型的一部分实现了 Drop trait, Rust 不允许它再去实现 Copy trait 
*/

// 实现了 Copy trait 的类型：
//  - 所有的整数类型(例如 u32)
//  - bool
//  - char
//  - 所有的浮点类型（例如 f64）
//  - tuple(元组), 如果其所有的字段都是 Copy 的
//
fn simple_copy_trait() {
    let x = 10;
    let y = x;
    println!("{x} - {y}");
}

// 像大多数存放在堆上的数据没有实现 copy trait, 因此在进行
// 变量赋值的时候

fn no_copy_trait() {
    let s = String::from("work");
    // 这样的操作实际上就是将 s 移动到了 y
    let y = s;
    
    // 此时 s 移动后已经无法再次使用了
    // println!("{y}-{s}");
}

// ======================================================================
// 引用和借用
// ======================================================================
// 
// 我们把引用作为函数参数这个行为叫做借用
//   - 借用也分为可变借用(mut) 和 不可变借用(默认)
//   - 可以通过创建新的作用域，来允许非同时的创建多个可变引用(例子)
//

// 计算字符串的长度
// 使用 & 表示引用，允许引用某些值而不取得其所有权
// 1. 有时候我们在处理一些变量的时候，并不想获取变量的多有权，
//   仅仅是想要对变量进行部分操作而不想将它移出当前作用域
//
fn calculate_len(data: &String) -> usize { 
    data.len()
}


// 在不同的作用域中同时创建多个可变引用
//
// 2. 在特定作用域内，对某一块数据, 只能有一个可变引用(这样做的好处就是可以在编译期防止数据竞争)
fn more_mut_reference() {
    let mut s = String::from("Hello");
    {
        let s1 = &mut s;
        change_str(s1);
        
    }
    let s2 = &mut s;
    change_str(s2);
}

fn change_str(s : &mut String) {
    s.push_str(", world");
}

// 3. 不可以同时拥有一个可变引用和一个不变的引用
//

fn cant_mut_with_imut() {
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    let s1 = &mut s;
    // println("{}, {}, {}", r1, r2, s3)
}

//
// 4. 可以拥有多个不可变引用


// ========================================================================
// 悬垂引用 (Dangling Pointer)
// ========================================================================
//
// 一个指针引用了内存中的某个地址，而这块内存可能已经释放并分配给其他人使用了
// 在 Rust 中，编译器可保证引用永远都不是悬垂引用.
//

// s 在离开作用域的时候已经被 drop 掉了，但是还返回了它的引用，这就产生了悬垂引用
// 这在 rust 中是无法被编译通过的
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
