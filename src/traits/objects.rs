// trait 对象
// ------------------------------------------------------------------------
// trait 对象的使用
//
// trait 对象通过指针来创建，如 & 或者 Box<T>(一种智能指针, 可以把数据存储在堆上)
// &dyn Trait or Box<dyn Trait>
// Box 是 Rust 中唯一一个可以直接把数据分配到堆上的类型

trait Animal {
    fn make_sound(&self) -> &str;
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) -> &str{
        "woof"
    }
}


struct Cat;

impl Animal for Cat {
    fn make_sound(&self) -> &str{
        "meow"
    }
}

pub fn union()  {

    let d = Dog;
    let c = Cat;

    let animals: Vec<&dyn Animal> = vec![&d, &c];
    animal_make_sound(animals);

}


// 通过引用和 trait 的方式来实现运行时的动态分发
// 因为 trait 对象的限定在编译期不能判断类型
// 因此在运行时需要先判定类型，然后根据类型来查找对应的方法，
// 这样会带来一定的运行时性能开销，但是灵活性更好
fn animal_make_sound(animals:  Vec<&dyn Animal>) {
    for ele in animals {
        ele.make_sound();
    }
}

// 3. trait object 安全 
// --------------------------------------------------------------
// (不是所有的trait都可以创建 trait object) 以下两种情况之外的类型都不能创建 trait 对象
// trait 中方法返回类型不为 Self
// trait 中方法不存在泛型类型参数
//

pub trait Yris {
    fn method(&self) -> Self;
}

// 泛型参数的trait
pub trait Xris {
    fn print<T: std::fmt::Display>(&self, t:T);
}

// 无法使用 trait object
// fn use_trait_object(t: &dyn Xris) {}
//  fn use_trait_object(t: &dyn Yris) {}




// 4. trait 实现与所有权
//
// 在自定义的 trait 方法中, 可以根据需要获取类型的不可变引用，可变引用或者所有权

// 所有权机制和 trait 本质上是两个独立的概念，即使没有 trait, 所有权机制也是成立的
// 这也就是在前面讨论所有权机制的时候并没有提及 trait,
// 但是 trait 让所有权机制更加显示化，更好理解，更好使用

trait A {
    // 需要手动实现，获取所有权
    // 对象执行此方法之后，将移交所有权，则对象不能再被使用
    fn take_ownership(self);

    // 默认实现，获取不可变引用
    fn take_ref(&self) {
        println!("此方法获取了不可变引用");
    }

    // 默认实现，获取可变引用
    fn take_mut_ref(&mut self) {
        println!("此方法获取了可变引用");
    }
}

struct X;

impl A for X {

    fn take_ownership(self) {
        println!("方法获取到类型的所有权");
    }
    // 默认实现不用手动实现
}


