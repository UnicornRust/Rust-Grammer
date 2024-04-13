//
// at 运算符允许我们在创建一个存放哪个值的变量的同时测试其值是否匹配模，
// 使用 `@` 可以在一个模式中同时测试和保存变量值。
//
pub fn run() {
    value_validate();
}

//
fn value_validate() {
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            // 如下我们在进行模式匹配的时候并对解构的值进行范围校验
            // 通过在 3..=7 之前指定 id_variable @, 我们捕获任何匹配此范围的值
            // 并同时测试其值匹配这个范围模式。
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),

        Message::Hello { id: 10..=12 } => {
            // 当前分支只在模式中指定了一个范围，没有一个包含 id 字段实际值的变量
            // id 字段的值可以是 10, 11, 12, 不过这个模式的代码并不知情野不能使用 id 字段的值
            // 因为没有将 id 值保存进一个变量
            println!("Found an id in another range");
        }
        // 最后一个分支没有指定对 id 值的一个范围，此时拥有可用于分分支代码的变量 id, 
        // 因为这里使用了结构体的字段简写语法，不过此分支中没有对 id 字段值进行限制测试
        // 因此任何值都会匹配。
        Message::Hello { id } => println!("Found some other id { }", id),
    }
}

enum Message {
    Hello { id: i32 },
}
