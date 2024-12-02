
pub fn fix_size_memory() {

    // 整数类型
    let _int_var: i32 = 10;
    println!("Size of integer: {}", size_of::<i32>());

    // 浮点数类型
    let _float_var: f64 = 10.0;
    println!("Size of float: {}", size_of::<f64>());

    // 布尔类型
    let _bool_var: bool = true;
    println!("Sise of bool: {}", size_of::<bool>());

    // 字符类型
    let _char_var: char = 'a';
    println!("Size of Char: {}", size_of::<char>());

    // 数组类型
    let _array_var: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Size of array: {}", size_of::<[i32; 5]>());

    // 元组类型
    let _tuple_var: (i32, f64, bool, char) = (10, 2.9, true, '0');
    println!("Size of tuple: {}", size_of::<(i32, f64, bool, char)>());

    // 引用类型
    let _reference_var: &i32 = &10;
    println!("Size of reference: {}", size_of::<&i32>());  // Size of reference: 8

    // 裸指针类型
    let _raw_pointer_var: *const i32 = &10;
    println!("Size of raw pointer: {}", size_of::<*const i32>()); // Size of raw pointer: 8

    // 函数指针类型
    let _function_pointer_var: fn() -> i32 = || 10;
    println!("Size of function pointer: {}", size_of::<fn() -> i32>()); // Size of function pointer: 8

}

// 总结：
// 对于固定大小的类型，将一个变量赋值给另一个变量时，实际上是为变量开辟了新的内存空间，并把值
// 拷贝了过去，也叫作 Copy 语义。
// 最终的结果就是出现一份新的值(与原来的值相同)以及其所有者
// 新的变量与原来的变量之间相互独立，互不影响


pub fn fix_size_own() {

    let num1 = 10;
    let num2 = num1;

    // 对应的有两个值和其对应的两个所有者
    // num2 虽然是 num1 的复制品，但是他仍然是一个有效的所有者
    println!("num1: {}", num1);
    println!("num2: {}", num2);

    // 可以观测到他们的地址是不同的，说明存在两个值, 两个所有者
    println!("num1 addr: {}", &num1);
    println!("num1 addr: {}", &num2); 

    // float , char 都是类似的情况


    // 对于复合类型，数组和元组

    let arr1: [i32; 3] = [1, 3, 4];
    let arr2: [i32; 3] = arr1;

    // 地址也是不同的
    println!("arr1 addr: {:p}", &arr1);
    println!("arr2 addr: {:p}", &arr2);

    // 元组类型
    // 元组类型的地址也是不同的
    let tuple1: (i32, i32) = (1, 2);
    let tuple2: (i32, i32) = tuple1;
    println!("tuple1 addr: {:p}", &tuple1);
    println!("tuple2 addr: {:p}", &tuple2);

    // 引用类型
    let num = 21;
    let p1 = &num;
    let p2 = p1;

    // 引用赋值进行了值的拷贝，地址也不同
    println!("p1 addr: {:p}", p1);
    println!("p2 addr: {:p}", p2);


    // 裸指针类型
    let num = 21;
    let p1 = &num as *const i32;
    let p2 = p1;

    // 裸指针类型进行了值的拷贝，地址也不同
    println!("p1 addr: {:p}", p1);
    println!("p2 addr: {:p}", p2);

    // 函数指针类型
    let num = 21;
    let p1 = || num;
    let p2 = p1;

    // 函数指针类型进行了值的拷贝，地址也不同
    println!("p1 addr: {:p}", &p1);
    println!("p2 addr: {:p}", &p2);

}
