
/// 错误处理的建议
/// 1. 编写示例，原型代码，测试(可以使用panic!())
///   - 演示某些概念 `unwrap`
///   - 原型代码: unwrap, expect
///   - 测试代码: unwrap, expect(某个方法调用失败, 则测试应该终止)
/// 2. 当你比编译器掌握更多的信息，明确知道结果
///   - "127.0.0.1".parse().unwrap(); // 明确知道肯定解析成功
///
/// 3. 当你的代码最终可能处于损坏状态的时候，最好是 panic!()
///    损坏状态(bad state): 某些假设，保证，约定或不可变性被打破
///    - 例如非法的值，矛盾的值或空缺的值被传入代码
///    - 以及下列中的一条
///      - 这种损坏窗台并不是预期能够偶尔发生的事情
///      - 在此之后，您的代码如果处于这种损坏状态就无法运行
///      - 在您使用的类型中没有一个好的方法来将这些信息(处于损坏状态) 进行编码
/// 
/// 4. 场景建议
///    - 调用你的代码，传入无意义的参数值 panic!();
///    - 调用外内部比可控代码，返回非法状态, 你无法修复 panic!()
///    - 如果失败是可预期的， Result
///    - 当你的代码对值进行操作，首先应该验证这些值的合法性，不合法直接 panic!()

pub fn run() {
    let guess = Guess::new(78);
    println!("{}", guess.value);
}

struct Guess {
    value: i32,
}

impl Guess {

    fn new(init: i32) -> Self {
        // 构建对象的时候，参数校验, 非法参数应该直接 panic , 
        // 这里字段是私有的，外部无法直接对字段直接赋值，必须要使用 new 
        // 也就是必须要经过验证，这样就可以保证安全性，对应其他语言中的 getter, setter 的设计
        if !(0..=200).contains(&init) {
            panic!("参数不在正常范围");
        }
        Guess { value: init }
    }
}
