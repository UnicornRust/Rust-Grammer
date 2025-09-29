
//-------------------------------------------------------
// 1. Cell<T>, RefCell<T>, OnceCell<T> 家族具有内部可变性
// 2. 由于没有实现 `Send` 这个 trait,所有只能适用于单线程
// ------------------------------------------------------
// 
// Cell<T> 实现内部可变性的方式
//
// 1. 通过移动（move) 值的方式来实现内部可变性
//   - 无法获取到内部值的 &mut T
//   - 也无法直接获取其内部的值, 除非用别的值替换它
// 2. 通过以上两条确保了不会有多个引用同时指向内部的值
// -----------------------------------------------------
//
// >> 常用方法
//
// 1. 针对实现了 Copy 的类型 
//   - get() 方法可通过复制的方式获取内部值
// 2. 针对实现了 Default 的类型
//   - take() 方法会将当前内部值替换为 Default::default(),并返回原来值
// 3. 针对所有的类型
//   - replace() 方法替换当前内部值，返回原来的值
//   - into_inner() 方法，消耗(consume) 掉这个Cell<T>, 并返回内部值
//   - set() 方法为其设置一个新的值，丢弃原来的值 
//
// >> 使用场景
//
// 1. Cell<T> 一般用于简单类型(如数值), 因为复制/移动不会太消耗资源
// 2. 在可能的情况下，优先使用 Cell<T>, 而不是其他的 Cell 类型.
// 3. 对于较大或者是不可赋值（non-copy)的类型，RefCell更有优势

use std::cell::Cell;


pub fn cell_run() {
   api(); 
}

fn api() {

    // get api 
    let cell = Cell::new(5);
    assert_eq!(cell.get(), 5);

    // replace 
    assert_eq!(cell.replace(10), 5);
    assert_eq!(cell.get(), 10);

    // into_inner 
    assert_eq!(cell.into_inner(), 10);
    // 消耗掉之后，就无法再次使用了
    // cell.get();

    // default
    let cell = Cell::new(String::from("hello"));
    // take 一次就是拿出原本的值, 换入默认值
    assert_eq!(cell.take(), "hello");
    // take 第二次就是拿出默认值
    assert_eq!(cell.take(), String::default());

    cell.set(String::from("world"));
    assert_eq!(cell.take(), "world");

    let cell = Cell::new(String::from("mifo"));
    // set 一个新值，丢弃旧值, set 没有返回值，默认为 ()
    // let x =  cell.set(String::from("peek"));
    cell.set(String::from("peek"));
    assert_eq!(cell.take(), "peek");
}
