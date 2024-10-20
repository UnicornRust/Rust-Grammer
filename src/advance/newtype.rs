use std::collections::HashMap;

// 测试新类型
pub fn run() {

    test_newtype();

}


// newtype 是是一个新的类型
// 可以对内部的数据类型提供封装的共有APi
#[derive(Debug)]
struct People {
    notes: HashMap<i32, String>,
    name: String,
    no: i32,
}

impl People {
    pub fn new() -> Self {
        Self {
            notes: HashMap::new(),
            name: String::from("alix"),
            no: 0,
        }
    }
    fn add_friends(&mut self, nickname: &str) {
        self.no += 1;
        self.notes.insert(self.no, nickname.to_string());
    }
}

fn test_newtype() {
    // 测试封装的新类型 People
    let mut p = People::new();
    p.add_friends("bell");
    p.add_friends("kica");
    p.add_friends("soot");
    println!("p: {p:?}")
}


// 为了安全和抽象而使用 newtype,
// 1. 静态的确保某个值不被混淆
//

