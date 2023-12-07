use crate::intelligent_point::box_type::List::{Cons, Nil};

// box 的使用
pub fn box_use() {
    // 声明一个 box 类型的变量,
    // 这里声明了一个 i32 类型的变量，他将存储在堆上(大多数情况下是非必要的)
    // 主要是展示一下怎么使用一个 box 类型的变量
    let x = Box::new(3);
    dbg!(x);

    // 在递归类型中，使用 box 来包裹递归的类型，
    // 可以在堆中存储数据，从而使得编译器可以计算这个递归类型的大小
    // 因为这时候这个 box 包裹的类型，需要的空间就是一个指针的大小
    recursive_type();

    // 像使用引用一样使用 Box<T>
    use_intellij_point_as_reference();
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 使用 box 来定义一个递归类型
fn recursive_type() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    dbg!("list = {} ", list);
}

// 像使用引用一样使用 Box<T>
fn use_intellij_point_as_reference() {


    // 变量 x 存放了一个 i32 值 5，y 等于 x 的一个引用，可以断言 x 等于 5
    // 然而，如果希望对 y 的值做出断言，必须使用 *y 来追踪引用指向的值(解引用)
    // 这样编译器就可以比较实际的值了, 一旦解引用了 y, 就可以访问 y 所指向的整型值
    // 并与 5 做比较J
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 像使用引用一样使用 Box<T>
    // 在当前的代码中，在 Box<T> 上使用解引用运算符
    // 与在 y 引用上使用解引用运算符具有一样的功能
    // 不同的是：这里 z 设置为一个指向 x 值拷贝的 Box<T> 实例。而不是指向 x 值得引用
    let z = Box::new(x);
    assert_eq!(5, *z);

}
