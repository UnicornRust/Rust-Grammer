use std::fmt::{Display, Formatter};

///  用于测试结构相关的基础知识点
pub fn struct_variable() {
    println!("------------------start test of struct----------------");
    declare_struct();

    // 结构更新语法
    update_from_other();

    // 没有字段的元组结构体
    tuple_struct();

    // unit struct
    unit_struct();

    // use scene
    get_rectangle_area();

    // 以字符形式输出一个结构体
    print_rectangle();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "username: {}, email: {}, active: {}, sign_in_count: {}",
                self.username, self.email, self.active, self.sign_in_count,
            )
            .as_str(),
        )
    }
}

fn declare_struct() {
    let mut user1 = User {
        active: true,
        username: "Ancionn".to_string(),
        email: "ancion@github.com".to_string(),
        sign_in_count: 1,
    };
    user1.username = "maria".to_string();

    let user2 = build_user("unicorn@gitbub.com", "unicorn");
    println!("user2: {}", user2)
}

fn build_user(email: &str, username: &str) -> User {
    User {
        active: true,
        email: email.to_string(),
        username: username.to_string(),
        sign_in_count: 2,
    }
}

fn update_from_other() {
    let user = User {
        active: true,
        email: "ancion@gitbub.com".to_string(),
        username: "ancion".to_string(),
        sign_in_count: 2,
    };

    // 结构更新语法，
    // 类似于 js 中的结构，但是原理完全不同，
    // 只能放在最后，同时只会把上面没有定义的字段放在后面
    let user2 = User {
        email: "unicorn@github.com".to_string(),
        ..user
    };

    // user 由于其中某些字段已经被移动，所以已经不能使用了
    // println!("user.name: {}", user.username);
    println!("user2.name: {}", user2.username);
}

///-----------------------------------------------------
/// 2.没有名字字段的元组结构体来创建不同的 类型
///-----------------------------------------------------

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_struct() {
    // black 和 origin 值得类型不同，因为他们是不同得元组结构体得实例
    // 定义得每一个结构体都有自己得类型，即使结构体中字段有着相同得类型
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", origin.1);
}

///-----------------------------------------------------
/// 3.没有任何字段的类单元结构体
///-----------------------------------------------------
/// 类单元结构体通常在想要某个类型上实现 trait 但不需要在
/// 类型中存储数据的时候发挥作用。
// 定义一个单元结构体，不需要（）或者 {}
struct AlwaysEqual;

fn unit_struct() {
    // 直接实例化，不需要 () 或者 {}
    let _subject = AlwaysEqual;
}

/// ----------------------------------------------------
/// 例子，什么时候使用结构体
/// ----------------------------------------------------

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 给结构体定义方法
impl Rectangle {
    // 专门给 Rectangle 的方法写在 Rectangle 的实现中，
    // 使得代码的聚合度更好，给 结构体定义方法的时候，
    // 第一个参数永远需要是 &Self,
    fn area_own(&self) -> u32 {
        self.width * self.height
    }
    // 关联函数，类似于静态方法, 没有 &self
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    // 如果想要获取到实例的所有权对数据进行修改
    fn double_width(&mut self) {
        self.width *= 2
    }
}

// 计算面积
// 这里定义的函数是给 Rectangle试用的, 但是写在函数之外，显得代码有点散乱
// fn area(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
// }

fn get_rectangle_area() {
    // 1. 原始版本
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels",
        width * height
    );
    // 2. 元组版本
    let rectangle = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        rectangle.0 * rectangle.1
    );
    // 3. 结构体版本，语意明确
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    // 需要使用到可变的引用，这时候在定义的时候需要讲变量定义为可变的，
    rect.double_width();
    println!(
        "The area of the rectangle is {}:{} square pixels",
        // 关联函数调用，使用结构体调用，类似于静态方法
        Rectangle::area(&rect),
        // 结构体的方法调用，使用实例调用
        rect.area_own()
    );
}

// 格式化输出一个结构体
fn print_rectangle() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    // 在结构体上添加  #derive(Debug) 实现很多的功能，
    // 这里只是使用了他的输出所有的属性方法
    // {:?} 可以直接接收对象打印，而不需要实现 Display 的实现
    // 使用 # 可以让格式更加好看
    println!("{rect:#?}");
    // 打印表达式，可以打印行号与文件位置
    // 这个表达式会接受所有权最后返回所有权
    dbg!(&rect);
}
