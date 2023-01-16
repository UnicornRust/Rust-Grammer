use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

/// 使用 Result 来处理潜在的错误
/// ------------------------------------------------------------
/// 很多时候，一个函数因为一个容易理解并做出反应的原因失败
/// 例如: 打开一个不存在的文件而失败，此时我们可能想要创建这个文件
/// 而不是直接终止进程
///
pub fn use_result() {
    println!("---------------test of use result--------------");
    //
    open_file();

    // unwrap
    unwrap_and_expect();

    // 错误的传播, 这里调用的函数可能将错误返回，所以调用者需要自己处理
    let result = propagating_error().expect("Read file content failure");
    println!("{:?}", result);

    // 错误传播的简化操作符号
    let result = propagation_error_operation().expect("Read file content failure");
    println!("{:?}", result);

    // 错误传播的简化后链式调用
    let result = propagating_error_chain_handle().expect("Read file content failure");
    println!("{:?}", result);

    // fs 模块中封装的读取文件的方法
    let result = simplified_read_file().expect("Read file content failure");
    println!("{:?}", result);

    last_char_of_first_line(&result);
}

fn open_file() {
    // enum Result<T, E>  {
    //   Ok (T),
    //   Err(E),
    // }

    // 文件打开操作返回的就是一个 Result
    let f = File::open("temp/hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 如果是文件不存在则创建文件
            ErrorKind::NotFound => match File::create("temp/hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file:{:?}", error),
            },
            // 非文件不存在的错误则 panic, 显示错误原因，终止程序
            other_error => panic!("Problem opening the file:{:?}", other_error),
        },
    };
    println!("{:?}", f);

    // 我们使用了大量的 match, 这是基本用法，后续学习了闭包的特性之后，
    // 可以使用很多 Result<T,E> 上定义的方法，使得程序更加简洁, 例如
    // 这里的操作与上述操作的效果是相同的 |error| {} 这就是闭包
    let file = File::open("temp/hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("temp/hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", file);
}

/// Rust 中封装了很多常见的处理 Result 的函数，方便我们进行一些模板样式的操作
/// unwrap 操作当正常返回的时候会返回 Ok 中的值，Err 时会自动 panic, 这里panic
///        给出的异常信息是程序内部处理的
/// expect 操作与 unwrap 基本是一样的，但是这里的错误的信息是我们自己定义的
///        方便后期追溯
fn unwrap_and_expect() {
    // 例如我们想要打开文件
    let f = File::open("temp/hello.txt").unwrap();
    println!("{:?}", f);
    // 可以自定义信息
    // let f = File::open("temp/hell.txt").expect("Filed to open hell.txt");
    // println!("{:?}", f);
}

/// 错误的传播, 我们除了可以在函数中处理错误之外, 还可以选择让调用者来处理这个错误
/// 因为相比你代码所拥有的上下文，代码的调用者可能有更多的信息来决定该如何处理错误
fn propagating_error() -> Result<String, io::Error> {
    // 例如我们要在一个文件中读取某个内容
    let f = File::open("temp/hello.log");
    let mut f = match f {
        Ok(file) => file,
        // 这里直接返回了错误, 这里之所以需要 return,而最终 match 表达式不需要 return，
        // 是因为这里不是最后一行表达式, 最后一行表达式默认是返回的
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/// 这样的操作可以使用错误传播操作符
/// ----------------------------------------------------------------------------
///  `?` 操作符 是用来减少错误传播的模板样式代码,其次, 它与 match 不同的是,
///  这个运算符将错误传递给了 from 函数，这个函数是 From trait 中定义的，
///  其是用来将一种错误转换为另一种错误的,当我们需要返回单个错误类型来代表所有可能
///  的失败的时候很有用。只要这些错误类型都实现了 from 函数，转换将被自动被执行
fn propagation_error_operation() -> Result<String, io::Error> {
    // 此处 ? 作用就是成功时 Ok 内的值赋给 f, 失败时提前返回 Err
    let mut f = File::open("temp/hello.txt")?;
    let mut s = String::new();
    // 这里的 ？作用是失败是返回 Err, 成功是 s 被赋值文件内容
    f.read_to_string(&mut s)?;
    Ok(s)
}

/// ? 后甚至可以使用链式调用来继续处理
fn propagating_error_chain_handle() -> Result<String, io::Error> {
    let mut s = String::new();
    // 这里在获取到文件描述符后使用链式调用来继续处理
    // 读取文件内容，
    File::open("temp/hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

/// 将文件读取到字符串中是如此常见，所以 Rust 为这个操作提供了函数
/// use std::fs
fn simplified_read_file() -> Result<String, io::Error> {
    fs::read_to_string("temp/hello.txt")
}

/**
  `?` 运算符使用场景: 只能被用于返回值与 ? 作用值相兼容的函数.因为 ? 运算
  符被定义为从函数中提早返回一个值。
 ------------------------------------------------------------------------
 > 只能在返回 Result 或者其他实现了 FromResidual 类型的函数中使用 ? 运算符
   如果 Err 有值，则Err 会从函数中提前返回;
   如果 Ok 有值, Result 中 Ok 中的值作为表达式的值，函数继续

 > 只能在返回 Option 的函数中对 Option<T> 使用 ?, 这个操作符的行为与 Result
   类似，如果值为 None，此时 None 会从函数中提前返回，如果值是 Some, Some中
   的值作为表达式的返回值同时函数继续.

  **__虽然 ? 可对以上两种类型的返回值的函数进行处理，但是不能混用，这两者之间
   是不能进行默认转换的，可以使用 Result 的 ok 方法或者 Option 的 ok_or 方法
   来进行显示转换__**
*/
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
