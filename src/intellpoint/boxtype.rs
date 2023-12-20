use crate::intellpoint::boxtype::List::{Cons, Nil};
use std::ops::Deref;

// box 的使用
pub fn run() {
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

    // 测试 Drop trait 执行的时机
    drop_trait_occasion();
}

struct CustomSmartPoint {
    data: String,
}

// Drop trait 执行的时机
// 修改 drop 实现，直接打印数据，查看对应的 drop trait 在什么时候会执行
impl Drop for CustomSmartPoint {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPoint with data `{}`!", self.data);
    }
}

fn drop_trait_occasion() {
    let _c = CustomSmartPoint {
        data: String::from("my staff"),
    };
    let _d = CustomSmartPoint {
        data: String::from("other staff"),
    };
    println!("CustomSmartPoint created!");
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

    // 使用自定义的类型
    let p = MyBox::new(x);
    assert_eq!(5, *p);
}

// 这里我们仿照 Box<T> 类型自定义一个 MyBox<T> 类型来体会 Box<T> 实现的原理
// MyBox 是一个包含T 类型元素的元组结构体，
// 定义一个结构体并声明类一个泛型参数 T, 因为我们希望其可以存储任何类型的值
// MyBox::new() 函数获取一个 T 类型的参数并返回一个存放传入值的 MyBox 实例
//
struct MyBox<T>(T);

impl<T> MyBox<T> {
    // 仿照 Box::new(x:T) -> Box(T)
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 为了实现 *运算符在我们自定义的类型上具有解引用的功能呢个，我们需要实现 Deref trait
// Deref trait 将类型像引用一样处理
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        todo!()
    }
}
