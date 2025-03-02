use std::fmt::{Debug, Display};

///测试 trait 概念于使用
pub fn run() {
    println!("---------------test of trait -----------------");
    // test use struct use trait method
    struct_use_trait_method();

    // 结构体中使用泛型时，为实现了特定的 trait 的类型
    // 添加方法测试
    generic_for_struct_method_selected();

    // 测试标准库中的 ToString blanket implementation
    use_blanket_implementations();
}

/**
 trait 概念
 ---------------------------------------------------------------------
- trait 是一系列的方法签名的集合，用于定义一个实现某些目的必须行为集合
   例如我们有一系列的结构体，他们分别存储了一些不同类型与属性字段。
   这些存储内容的结构需要一个共同的行为，提供一个总结性的描述, 外部可以
   通过这个实例的这个行为获取到这部分信息，对于多个部分共享的行为的抽象
   就是 trait 的本意
   -------------------------------------------------------------------
    > trait 关键字定义
    > { } 包含所有方法签名组
    > 每个方法签名占用一行，使用; 结尾
*/
pub trait Summary {
    // 定义的方法签名
    fn summarize(&self) -> String;

    // 定义的默认方法，很多时候我们为方法提供默认实现是很有用的，这时候
    // 我们的结构体只需要实现这个 trait, 就可以使用这个默认实现，当然也
    // 可以复写方法实现，
    // ------------------------------------------------------------------
    // 我们在默认实现中可以调用没有默认实现的方法,
    fn summer_author(&self) -> String {
        format!("who knows, maybe you interest at : {}", self.summarize())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub usename: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

/**
 * trait 实现的要求，
 * ---------------------------------------------------------------------------
 *   > trait 和 实现 trait 的类型至少需要有一个在crate本地作用域中, 例如
 *      > 可以为crate本地作用域中的 tweet 实现标准库 的 Display trait
 *      > 可以为标准库中 Vec<T> 实现本地 crate 中的 Summary trait.
 *      > 不能为 Vec<T> 实现 Display trait, 因为这两者都不在本地crate作用域中
 */
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.usename, self.content)
    }
}

/// 测试结构体实现 trait 的用法
fn struct_use_trait_method() {
    let tweet = Tweet {
        usename: String::from("hourse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

/**
 * 如何使用 trait 来定义一个函数，可以接收多种不同类型的参数
 * --------------------------------------------------------------------------
 *
 *  > fn notify<T: Summary>(item: &T) {} 这样的模式称为 trait bound
 *  > fn notify(item: &impl Summary) {}  这样的模式称为 impl trait
 *
 * 与上述的方法相同，下面的方法其实是这样的标准写法的一种简短模式下的语法糖,
 * 当情况比较简单的时候，这样简短的写法更加的直观，但是复杂场景下很难表达或者
 * 无法表达
 *
 * 我们定义 item 为任何实现了 Summary trait 的类型
*/
pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// 表达的是 item1 和 item2 都需要实现 Summary trait
fn notify_1(item1: &impl Summary, item2: &impl Summary) {}

// 这里函数定义中，表达了参数必须要实现 Summary trait
// 同时两个参数是同一种类型, 上述的定义就不能表达两个
// 参数需要是同一种类型这种语义
fn notify_2<T: Summary>(item1: &T, item: &T) {}

/// 实现多个 trait, 使用 + 来连接
/// ------------------------------------------------------------------------
///
fn notify_3(item: &(impl Summary + Display)) {}

// trait bound 写法
fn notify_4<T: Summary + Display>(item: &T) {}

/// 通过 where 简化 trait bound
/// -------------------------------------------------------------------------
/// 当函数具有多个泛型参数，而每个泛型参数都有多个 trait bound 的时候
/// 函数名与参数列表之间有很长的 trait bound 信息，代码难于阅读
/// Rust 在这里引入了 where 子句，在函数签名之后使用
fn some_fun<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

// > where 子句的形式书写函数签名
fn some_fun_1<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

/**
 * trait 在返回值中的使用
 * --------------------------------------------------------------------------
 *  也可以在 函数返回值中使用 impl trait 语法, 表示函数的返回值需要是某个实现了
 *  summary trait 的类型。
 *  impl trait 在闭包和迭代器中非常有用，因为在闭包和迭代器中创建的类型只有编译器
 *  知道，或者是一个非常复杂的类型，可以返回一个 impl trait 允许你简单的指定函数
 *  返回一个 Iterator 而无需写出一个冗长的类型
 */
fn return_summarizable() -> impl Summary {
    Tweet {
        usename: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know people"),
        reply: false,
        retweet: false,
    }
}

// 下面这样的代码是不能编译使用的，一个指定返回 impl Summary 的类型返回了 Tweet 或者 NewsArticle
// 这与 impl trait 工作原理不相符,
/*
fn return_summary(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the stanley Cup Champinship!"),
            location: String::from("Pittsbutgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            usename: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know people"),
            reply: false,
            retweet: false,
        }
    }
}
*/

/**
 * 我们在处理带有泛型参数的类型的时候，可以使用 trait bound 语法
 * 为某些实现了指定 trait 的类型添加方法
 */
struct Pair<T> {
    x: T,
    y: T,
}

// 为所有的泛型种类都提供的 new 方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 为只有 实现了 PartialOrd 和 Display 的类型提供的方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest menber is x={}", self.x);
        } else {
            println!("The largest menber is y={}", self.y);
        }
    }
}

fn generic_for_struct_method_selected() {
    let some = Pair::new(Some("hello"), Some("ok"));
    println!("{:?}:{:?}", some.x, some.y);
    let int = Pair::new(20, 40);

    // 无法调用 cmp_display
    // some.cmp_display()

    // i32 实现了 display 和 PartialOrd 则可以调用
    int.cmp_display();
}

/**
* 可以为实现了特定的 trait 类型有条件的实现其他的 trait 为满足特定的
* trait bound 的类型实现 trait 称为 blanket implementations
* ---------------------------------------------------------------------
*  > 例如标准库中为所有实现了 Display trait 的类型实现了 ToSting trait
*   imple<T: Display> ToString for T {
*      // --snip --
*   }
*/
fn use_blanket_implementations() {
    //正是由于标准库中有了这些 blanket implementation,
    // 则实现了 Display trait  的类型都可以调用 to_string() 方法
    let s = 3.to_string();
    println!("3 to_string() is {s}")
}
