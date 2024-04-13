//
// unsafe 代码
//
pub fn run() {
    create_raw_point();
}

// 创建裸指针
//
fn create_raw_point() {
    // 在安全的代码中可以创建裸指针，只是不能在不安全块之外解引用它们
    let mut num = 5;
    // 这里使用 as 将不可变引用和可变引用强转为对应的裸指针类型
    // 因为在保证安全的引用创建，可以说这些特定的裸指针是有效的，
    // 但是不能对任何裸指针做出如此假设
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 现在我们需要解引用了，需要使用 unsafe 块包裹解裸引用的代码
    // 创建一个指针不会造成任何的危险，只有访问其指向的值才有可能遇到无效的值。
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // 创建一个不能确定有效性的裸指针，
    // 创建一个指向任意内存地址的裸指针，尝试使用任意内存是未定义行为
    // 这里可能有数据，也可能没有，编译器可能会优化这个内存访问或者出现段错误
    // segmentation fault (通常不要编写这样的代码尽管它是可行的)
    let address = 0x012345usize;
    let r = address as *const i32;
}
