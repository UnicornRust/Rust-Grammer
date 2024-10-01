use std::{thread, time::Duration};

/// 这一节主要展示 Rust 中 闭包的使用，
/// ------------------------------------------------------
/// 在 rust 中闭包主要是指可以存储在变量中的类似函数结构的匿名函数,
/// 或者作为函数参数的匿名函数.
/// > 大多数的情况下, 闭包可以不用定义数据类型，因为闭包通常很短，在
///    这样一个相对小的空间中，Rust 大多数情况可以通过推断确定类型.
/// > 闭包允许捕获被定义时所在的作用域中的值。

pub fn run() {
    println!("--------------------- test of closure ------------------");
    // 闭包概念的第一个例子
    first_closure();
    // 自动类型推导
    auto_type_infer();
    // 捕获不可变引用
    catch_reference();
    // 闭包捕获可变引用
    catch_mut_reference();
    // 闭包移动变量的所有权
    move_ownship();
    // 测试闭包的 trait
    closure_trait();
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // 用于分派 T-shirt, 返回分发的 T-shirt 颜色
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // 这里 most_stocked() 作为闭包被作为参数传递给 unwrap_or_else()
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

///
/// 1. 闭包的定义与使用
/// ----------------------------------
/// > 闭包允许捕获被定义时所在的作用域中的值. 这里有一点特殊的是
/// 我们在调用闭包的时候，使用的是一个 Iventory 实例的方法，

pub fn first_closure() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref = Some(ShirtColor::Red);
    let giveaway_pref = store.giveaway(user_pref);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref, giveaway_pref,
    );

    let user_no_pref = None;
    let giveaway_no_pref = store.giveaway(user_no_pref);
    println!(
        "The user with preference {:?} gets {:?}",
        user_no_pref, giveaway_no_pref,
    );
}

///
///  2. 自动进行类型推断
/// ---------------------------------------------------------------
/// 指闭包一般在一个关联的小范围的上下文，在这个又限制的上下文中，编译器
/// 可以可靠的推断参数和返回值的类型，(编译器需要闭包类型注解的情况及其罕见)
fn auto_type_infer() {
    let expensive_closure = |num| -> u32 {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    println!("{}", expensive_closure(20));
}

/// 闭包与函数定义的比较
/// ---------------------------------------------------------------、
fn add_one_fun(x: u32) -> u32 {
    x + 1
}
//
// > 以下这些都是闭包的定义，在定义闭包的时候，大多数类型可以推断的时候
//   可以省略类型注解, 当闭包中的函数体只有一个表达式的时候可以直接省略 `{}`
//
// > 要想下面的这些代码被编译的条件就是需要在程序中将这些部分作为闭包来
//   使用： 因为只有在使用的场景中 rust  才能推断这些参数与返回值的类型.
//
// > 一旦一个闭包被调用，推断出的类型将赋予给闭包中的参数或返回值
//   意味着我们不能使用其他类型来调用，否则会得到一个错误.
//
// const add_one_v1 = |x: u32| -> u32 { x + 1 };
// const add_one_v1 = |x| { x + 1 };
// const add_one_v1 = |x|  x + 1 ;
//

//
//  3. 捕获引用或者移动所有权
// --------------------------------------------------------------------
// 闭包可以通过三种方式捕获其环境，他们直接对应到函数获取参数的三种方式：
// - 不可变借用，
// - 可变借用
// - 获取所有权
// 闭包会根据函数体中如何使用被捕获的值来决定采用什么方式捕获.
//
// 以下的例子就展示了闭包捕获一个 list 的 vector 的不可变引用，因为只需要
// 不可变引用就可以打印其值。
fn catch_reference() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // 这里同时也展示了在 变量中保存闭包，然后使用变量名像使用函数名一样来调用这个闭包
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure {:?}", list);
}

// 我们再来捕获一个可变引用
fn catch_mut_reference() {
    let mut list = vec![1, 2, 3];
    println!("before defining closure: {:?}", list);
    // 在 vector 中添加元素, 这里我们定义的闭包需要的是一个可变的引用,
    // 我们在定义中捕获了可变应用，直到这个闭包调用结束，借用结束
    // 这就意味着在定义闭包到调用闭包之间不能有任何其他引用的出现
    // 因为在一个可变应用被持有的时候，不能出现其他引用
    let mut borrows_mutably = || list.push(7);
    // 调用之间不能出现其他的对 list 的引用
    borrows_mutably();
    println!("After call closure: {:?}", list);
}

// 所有权变更
// 当我们定义的闭包体自动捕获的的引用没有达到我们的预期的时候，
// 例如实际闭包中只需要一个不可变引用，但是我们期望这个闭包得到变量的所有权
// 这时候需要主动使用 `move` 关键字来移动变量，改变变量的所有权
fn move_ownship() {
    let list = vec![1, 2, 3];
    // 将一个闭包传递到新的线程中时，改变变量的所有权是非常必要的，
    // 可能存在主线程结束了，变量被清理，但是其他线程并没有结束的情况
    // 如果不移动变量，可能导致程序中存在不安全(无效)的引用
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

///
/// 4. 将捕获的值移出闭包 和 Fn trait
///   > 一旦闭包捕获了定义它的环境中的一个值的引用或者所有权（也就影响了什么会被移进闭包,如果有的话）
///   > 闭包体中的代码定义了在闭包计算时对引用或者如何操作（也就影响了什么会被移出闭包，如果有的话）
/// --------------
/// 闭包体可以做出以下任何事：
///   - 将一个捕获的值移出闭包，
///   - 修改捕获的值
///   - 既不移动也不修改值，
///   - 一开始就不从环境中捕获值
/// ---------------
/// 闭包捕获处理环境中的值的方式影响闭包实现的 Trait, Trait 是函数和结构体指定他们能用的闭包
/// 的类型的方式, 取决于闭包体如何处理值，闭包自动，渐进式地实现一个，两个或者 三个 Fn Trait.
///   - FnOnce : 适用于被调用一次的闭包，所有闭包都至少实现了这个 trait, 因为所有的闭包都能被
///       调用。一个会将捕获的值移出闭包体的闭包只实现 FnOnce trait, 这是因为它只能被调用一次.
///   - FnMut : 适用于不会将捕获的值移出闭包体的闭包，但它可能会修改被捕获的值，这类闭包可以被
///       调用多次。
///   - Fn : 适用于既不将捕获的值移出闭包体也不修改被捕获的值的闭包，当然也包括不从环境中捕获值
///       的闭包，这类闭包可以被调用多次而不改变他们的环境，这在会多次并发调用闭包的场景中十分重要。
///
fn closure_trait() {
    println!("closure trait");
    closure_fn_mut_trait();
}
// 4.1 : Option.unwrap_or_else(self, f: F) where F: FnOnce -> T
// ----------------------------------------------------------------
// 我们对于标准库中的 Option.unwrap_or_else() 中传入的闭包实现进行分析
// 这里就规定了这个传入的闭包至少实现了 FnOnce trait, 这就意味着
// unwrap_or_else() 至多对闭包代码调用一次，由于所有的闭包都实现了这个
// 因此这个方法可以接受大多数不同的类型的闭包，十分的灵活.
// ----------------------------------------------------------------
//  > 当一个闭包做的事情不需要从环境中捕获值得时候，则可以在需要 Fn trait
//    的东西时使用函数而不是闭包，
// ----------------------------------------------------------------
// impl<T> Option<T> {
//    pub fn unwrap_or_else<F>(sled, f: F) -> T
//    where F: FnOnce() -> T
//    {
//        match self {
//            Some(x) => x,
//            None => f(),
//        }
//    }
//}

// ---
//  4.2. Slice.sort_by_key(f: F) where F: FnMut() ->
// -----------------------------------------------------------------
// 标准库中的 Slice 上有一个 sort_by_key 方法，它需要接收的是一个 FnMut 类型的闭包
// 这个闭包以 一个 slice 中当前被考虑的元素引用作为参数，返回一个可以用来排序的 K 类型的值
// 当你想要按照 slice 中元素的某个属性来排序的时候，这非常有用。
//
// 例如当你想要使用 Rectangle 中 宽度排序时, 这里需要 FnMut trait 类型闭包的原因是在闭包内需要
// 多次调用，但是它并不捕获或者移出任何的变量。所以它满足 trait bound 的要求.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn closure_fn_mut_trait() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 1,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    //
    // 当我们修改这里的闭包，完成对闭包调用次数的计数, 我们在环境中定义了一个 value
    // 然后在闭包中捕获了 value，并在闭包中移动了 value 的所有权，因此这个闭包只实现了
    // FnOnce, 由于要求闭包至少实现 FnMut, 因此编译的时候便会报错, 报错的原因是：
    //  第一次执行闭包的时候已经移走了value 的所有权，再次执行闭包的时候，
    //  由于 value 已经不在作用域了，无法移动必然报错.
    // -------------------------------------------------
    // let value = String::from("call by key");
    // list.sort_by_key(|r| {
    //     sort_operation.push(value);
    //     r.width
    // });

    // 改变思路，我们并不移动变量的所有权，而是在作用域内维护一个
    // 累加器，让闭包捕获变量然后实现累加效果，从而计算闭包被调用的次数
    let mut sort_operation_count = 0;
    list.sort_by_key(|r| {
        sort_operation_count += 1;
        r.width
    });
    println!("{:#?}", list);
    println!("{:#?}, sorted in {sort_operation_count} operations", list);
}
