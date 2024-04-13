use std::fmt::{self, Debug};
use std::ops::Add;
use std::option::Option;

// 测试高级 trait
pub fn run() {
    // 判断 add 方法的有效性
    assert_eq!(
        Point{x: 1, y: 0} + Point {x: 2, y: 3},
        Point{x: 3, y: 3},
    );

    // 使用全限定方法名消除歧义
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    // Human::fly(&person);
    person.fly();

    // 函数全限定名称消除歧义
    // 1. 存在同名的关联函数，优先调用自身的
    println!("A baby dog is called a {}", Dog::baby_name());
    // 2. trait Animal 发起调用的时候，无法判定是哪个实现在调用
    //    出现编译错误
    // println!("A baby dog is called a {}", Animal::baby_name());
    // 3. 如果需要调用某个 trait 中的同名关联函数，需要使用全
    //    限定名称来完成对应的调用
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());


    // 4. 严重 Point outlint_print
    let p = Point{x: 1, y: 3};
    p.output_print();

    // 验证wrapper newtype
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w)

}


struct Counter {
    x: u32,
}

// 关联类型
// 对于 Counter 来说只能在实现的时候确定一次 Item 类型
// 如果是泛型，可以有多个类型实现 Iterator trait, 每次使用的时候需要
// 传入对应的类型来决定是调用哪一个实现
impl Iterator for Counter { 
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.x)
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
struct Point{ 
    x: i32,
    y: i32,
}


// 默认泛型类型参数
// Add 使用了默认的泛型类型参数 Rhs=Self
// 这是比较典型的例子，用于操作符的重载
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


// 不使用默认泛型参数的例子
// 在实现的时候传入对应的泛型参数覆盖默认的泛型
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


// 完全限定语法
// 实现的多个 trait 中有相同方法名称

trait Pilot {
    fn fly(&self);
}

trait Wizard{
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}
impl Wizard for Human {
    fn fly(&self){
        println!("UP!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously");
    }
}

// 当我们 trait 内是方法的时候，可以获取一个 self 参数
// 当有多个类型实现了同一个 trait 的时候，Rust 可以根据
// self 的类型来计算出应该使用哪一个 trait 实现
// --------------------------------------------------------
// 然而，不是方法的关联函数没有 self 参数
//
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

// 结构体自身有一个关联函数
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
// 实现的 trait 中有同名函数
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}


//
// 存在一个 trait 存在对另一个 trait 的依赖
// 需要实现类在实现当前 trait 时同时需要实现当前trait 依赖的 trait
//
// OutlinePrint 依赖 Display trait
trait OutlinePrint: fmt::Display {
    fn output_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}


impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}




// newtype 模式用以在外部类型上实现 trait
// 一些定义在 crate 之外的内容，想要对其实现一些其他 trait 
// 但是这在 rust 中这是不允许的。我们需要在当前 crate 中对其进行包装
//
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}
