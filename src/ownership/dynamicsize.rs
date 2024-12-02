// 动态类型
//
// 动态类型的大小，随着应用的执行，他的大小，内存地址都有可能是变化的
//
//
pub fn dyn_size_ptr() {

    // 1. &str
    let mut a: &str = "rust";
    let size_of_a = size_of_val(a);
    let ptr_of_a = a.as_ptr();

    println!("Size of string data: {} bytes", size_of_a);
    println!("Address of string data: {:p}", ptr_of_a);

    a = "go";

    let size_of_a = size_of_val(a);
    let ptr_of_a = a.as_ptr();

    println!("Size of string data: {} bytes", size_of_a);
    println!("Address of string data: {:p}", ptr_of_a);

    let ptr_rust = "rust".as_ptr();
    // 这里的地址与 ptr_of_a 第一次指向 rust 的时候是一样
    // 字符串字面量被编码到二进制文件中，指向的地址空间是不会发生变化的
    println!("Address of string data: {:p}", ptr_rust); 
    // 



    // 2.String 字符串类型在修改前后的值的变换和内存地址的变化
    let mut string_data = String::from("hello world");
    let size_of_string = string_data.len();
    let ptr_of_string = string_data.as_ptr();

    println!("Size of string data: {} bytes", size_of_string);
    println!("Address of string data: {:p}", ptr_of_string);

    let size_of_string = string_data.len();
    let ptr_of_string = string_data.as_ptr();

    println!("Size of string data: {} bytes", size_of_string);
    println!("Address of string data: {:p}", ptr_of_string);

    // 3. Vec 类型的变化
    // vec 内存的地址的变化是因为当 vec 中元素存放的数量超过了上一次分配空间
    // 的占用的阈值，为了能存下更多的数据，需要重新申请一块更大的内存(新的内存地址)
    let mut vec_data = vec!["1"];
    let size_of_vec = vec_data.len();
    let ptr_of_vec = vec_data.as_ptr();

    println!("Size of vector data: {} bytes", size_of_vec);
    println!("Address of vector data: {:p}", ptr_of_vec);

    vec_data.push("2");
    vec_data.push("3");
    vec_data.push("4");
    vec_data.push("5");
    vec_data.push("6");
    vec_data.push("7");
    vec_data.push("8");
    vec_data.push("9");
    vec_data.push("10");

    let size_of_vec = vec_data.len();
    let ptr_of_vec = vec_data.as_ptr();
    println!("Size of vector data: {} bytes", size_of_vec);
    println!("Address of vector data: {:p}", ptr_of_vec);

}



// 总结：
// > 当数据存放在堆上时，将一个变量赋值给另一个变量，意味着把堆上所有权转移给新的所有者,
//   堆上的数据本身没有发生复制，原来的所有者不再拥有数据，就是 Move 语义(所有权的 Move)
//
// > 当数据存放在栈上时，将一个变量赋值给另一个变量，意味着把栈上的数据复制了一份给新的变量
//   原来的变量不受新变量的影响，就是 Copy 语义， 这是指值的 Copy

// 动态类型的所有权是 rust 中的难点，
// 因为动态大小的类型，无法在编译期间知道他的大小，只有在运行期间才能知道
// 又因为栈的大小比较小，因此动态大小类型的内存分配一般是在堆上的
//
pub fn dyn_own() {

    // 1. 字符串与所有权
    // 一般情况下，字串串字面量存放在程序的只读数据段中，声明后很少去修改它
    // 而需要动态变化的字符串我们会把它存放到堆上，并且通过栈内存来管理堆内存

    let ptr_owner = "Rust"; // 存放在只读数据段中
    let mut heap_ptr_owner = String::from("Rust"); // 存放在堆上
    //
    // 1.1 对于存放在只读数据段中的字符串字面量，他的所有权和其他基本类型一样
    //
    let ptr_copy = ptr_owner;

    // 由于ptr_copy 和 pre_owner 都是指向相同值的引用，所以他们指向的内存地址是相同的
    println!("ptr_copy addr: {:p}", &ptr_copy);
    println!("ptr_owner addr: {:p}", &ptr_owner);


    // 1.2 对于存放在堆上的字符串
    let heap_ptr_new = heap_ptr_owner;

    // 上面的语句已经将 heap_ptr_owner 的所有权转移给了 heap_ptr_new
    // 所以 heap_ptr_owner 已经指向了一个无效的值，无法使用
    // println!("heap_ptr_new addr: {:p}", &heap_ptr_owner);
    println!("heap_ptr_new addr: {:p}", &heap_ptr_new);

    {
        let owner_old = String::from("rust");
        // 所有权移交
        let owner_new = owner_old;
        // 所有权被清理
    }
    // owner_new 虽然拿到了所有权，当时走出了作用域之后被清理，无法再使用
    // println!("owner_new addr: {:p}", &owner_new);


    // 被移交所有权的标识符，无法使用是因为所有权移交后被标注为空，不是立即清理掉了，
    // 虽然无法进行其他操作，但是如果被赋予新的值(标识符遮蔽) 不建议这么使用，代码难以阅读
    heap_ptr_owner = String::from("Go");
    println!("heap_ptr_owner addr: {:p}", &heap_ptr_owner);


    // 2. 所有权与 slice
    // 上面的字符串 str 实际上是一个特殊的 slice, 它仅代表有序的 utf-8 序列
    // 而切片中可以包含任何类型的元素，如其他基础类型，自定义类型等。
    // 正如不直接使用 str 一样，我们也不直接使用 [T], 而是使用他的指针(引用) 类型 Vec<T>
    // slice 中的数据也存放在堆上，Rust 中 slice 内存管理逻辑同存放在堆上的 str.

    let str_slice = vec!["rust", "go", "cpp"];
    let u32_slice: Vec<i32> = Vec::new();
    
    let new_owner1 = str_slice;
    let new_owner2 = u32_slice;

    // 一旦发生赋值操作，就意味着所有权被移交，无法再使用
    // println!(": {:p}", &str_slice);
    // println!(": {:p}", &u32_slice);

    // 可以通过新的所有权获得者来访问原来的数据
    println!(": {:p}", &new_owner1);
    println!(": {:p}", &new_owner2);


}







