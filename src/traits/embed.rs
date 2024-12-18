//
// ==============================================
// 1.Copy and Clone trait
// ==============================================
//
// 大部分的固定大小类型都实现了 Copy trait, 赋值行为发生时进行了值的复制

use std::ops::Deref;

fn copy_clone() {
    // 
    let owner_one = 32;
    let owner_two = owner_one;

    println!("one: {}, two: {}", owner_one, owner_two);

    // 对于一些动态大小的类型，比如 str, [T], 在使用他们的指针 String, Vec<T> 时
    // 不会发生只的复制，而是发生所有权的转移

    let owner_one = String::from("hello");
    let owner_two = owner_one;

    // 所有权发生转移后 owner_one 已经被标记为无效了      
    // println!("one: {}, two: {}", owner_one, owner_two);


    // 从 trait 角度来讲，就是固定大小类型实现了 Copy trait 和 Clone trait, 
    // 动态大小类型没有实现 Copy trait, 大多数实现了 Clone trait
    // 经常在一些错误使用的时候，编译器会告诉你，这些类型没有实现 Copy trait
    
    // 如果你想要在堆上赋值, 想像固定大小类型一样复制一份数据，
    // 可以显示调用 Clone trait de  clone() 方法


    // clone 出的数据都有自己的所有权，都是独立有效的, 不会发生所有权转移 
    let v = vec![1, 2, 3, 4, 5];
    let v1 = v.clone();
    let v2 = v1.clone();
    let v3 = v.clone();

}


// ==============================================
// 2.类型转换实例 From 和 Into Trait
// ==============================================

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number  {
    fn from (item: i32) -> Self {
        Number { value: item}
    }
}

pub fn from() {

    // 使用 From trait 中的 from 方法实现 i32 类型向 Number 类型的转换
    let num = Number::from(10);
    println!("Number is {:?}", num);

    let number = Number { value : 10};
    let i = i32::from(number);
    println!("i is : {}", i);
    
}


impl From<Number> for i32 {
    fn from(value: Number) -> Self {
        value.value
    }
}

pub fn into() {

    // 实现了 From trait 的类型就自动实现了 Into trait, 也可以使用 into() 方法
    // 注意使用 into() 的时候需要明确的指定返回的类型
    let n: Number = 23.into();
    println!("Number is {:?}", n);

    let number = Number { value : 10};
    let i: i32 = number.into();
    println!("i is : {}", i);

}

// 与 From 和 Into trait 类型的 还有 TryFrom 和 TryInto
// 在这个 trait 的实现中，增加了对错误的处理
// 业务场景中使用的更多


// ==============================================
// 3.AsRef 和 AsMut Trait
// ==============================================
//
// 通过 AsMut 可以获取结果提成员的可变引用
impl AsMut<i32> for Number {
    fn as_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
}

fn as_ref_mut() {
    let mut number = Number{ value: 20};
    let ref_num = number.as_mut();
    *ref_num += 10;
    println!("number is : {}", number.value);

    // 不可变引用
    let num = Number { value : 40};
    let ref_num = num.as_ref();
    println!("ref_num is : {}", *ref_num);
}

// 可以获取成员的不可变应用
impl AsRef<i32> for Number {
    fn as_ref(&self) -> &i32 {
        &self.value
    }
}

// ===============================================
// 4. 迭代器 Iterator / IntoIterator Trait
// ===============================================
// 
// 4.1. Iterator Trait 内部有
//  
//  一个关联类型 Item
//  迭代器的一系列方法, next, count, last, nth, chain...
//  
//
// 4.2 IntoIterator 中有两个关联类型
// 
//  一个是 Item
//  一个是 IntoIter(实现了 Iterator trait 的类型),
//  一个  into_iter(self) -> self::IntoIter 方法, 返回实现 Iterator trait 的类型

// rust 中 的 for 循环就是迭代器的语法糖

fn iterator_for() {
    let values = vec![1, 2, 3, 4, 5];
    // values 迭代之后就相当于被 move 掉了
    for x in values {
        println!("{x}");
    }

    // 等价效果
    let values = vec![1, 2, 3, 4, 5];
    let mut v_iter = values.into_iter();
    loop {
        match v_iter.next() {
            Some(x) => println!("{x}"),
            None => break
        }
    }
}


// ==============================================
// Deref 和 Drop trait 
// ==============================================

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}

impl Drop for User {
    fn drop(&mut self) {
        // 实现细节只是打印观测执行效果
        println!("User has been dropped: {:?}", "rust");  
    }
}


#[derive(Debug)]
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn drop_user_trait() {
    let mut user = User {
        name: String::from("Alex"),
        age: 12,
    };
    // 无法进行手动调用，而是在类型走出作用域时自动调用
    // 显式调用会冲突
    // user.drop()  
    // 走出作用域可以看到 drop() 函数中的打印执行了
}


fn dref_box() {
    let m = MyBox("rust");

    // 实现 Dref trait 的智能指针可以使用 * 解引用Box内部的值
    // 下面两种写法是等价的(直接解引用实际就是编译器调用deref()方法的语法糖)
    let ref_my_box = *m;
    let ref_my_box = (m.deref());
    
    println!("valus is {ref_my_box}");

    // String 实现了 Deref trait, 因此他也是智能指针，可以直接解引用
    let value = String::from("rust");
    take_string_ref(&value);

}

fn take_string_ref(v: &str) {
    println!("valus is {v}");
}
