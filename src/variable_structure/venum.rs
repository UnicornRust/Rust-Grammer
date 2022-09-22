pub fn test_enum() {
    test_first_level();
    test_second_level();
}

/// 枚举中只是列举了集合中的元素，这些元素中没有携带自己的信息
/// 这样使得我们需要定义其他的结构体来使得我们程序可读性更好
enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn test_first_level() {
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}


// ----------------------------------------------------------------

/// 我们在枚举中定义枚举自己具有的信息

enum IpAdders {
    // 为了方便使用， 语义明确，可以给让这个描述信息变成一个结构体
    // V4(String),
    // V6(String),

    // 一个枚举类中成员的描述信息的结构可以不同
    V4(u8,u8,u8,u8),
    V6{
        address: String
    },
}

fn test_second_level() {
    let _home = IpAdders::V4(127, 0, 0, 1);
    let _loopback = IpAdders::V6 { address: String::from("127.0.0.1") };
}

// ---------------------------------------------------------------

/// 我们可以在一个枚举类中定义完全不同结构的元素
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 与结构体一样。可以为枚举类型定义自己的方法。
impl Message {
    fn call(&self) {
        // 定义方法体
    }
}
