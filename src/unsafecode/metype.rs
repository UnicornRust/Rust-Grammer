use std::{collections::HashMap, marker::PhantomPinned, pin::Pin, ptr};

const DEFAULT_VALUE: &str = "";

pub fn engine_run() {
    let map = HashMap::from([
        (String::from("male"), String::from("Handsome boy")),
        (String::from("female"), String::from("Pretty girl")),
    ]);

    let engine = MySelfRefEngine::new(map);
    println!("Translate result: {}", engine.translate("male"));

}

struct MySelfRefEngine {
    map: HashMap<String, String>,
    translator: PtrTranslate,
    // 使得 MySelfRefEngine 成为 !Unpin
    _maker: PhantomPinned
}

impl MySelfRefEngine {
    // 为了保证结构的稳定，map 地址不应发生变化
    pub fn new(map: HashMap<String, String>) -> Pin<Box<Self>> {
        let mut engine = Box::pin(Self {
            map,
            // 这里不能直接获取 map 的裸指针, 因为这里 map 被移动，地址可能发生变化
            // 因此需要使用 pin 结构来固定 map (pin住之后，这里获取不到 map 指针,
            // 使用ptr::null()占位)
            translator: PtrTranslate { map: ptr::null() },
            _maker: PhantomPinned
        });

        let map_ptr: *const HashMap<String, String> = &engine.map;
        engine.translator.map = map_ptr;
        return engine;
    }
    fn translate(&self, text: &str) -> &str {
        // 如果 MySelfRefEngine 和 PtrTranslate 都需要使用 map
        println!("map size: {} ", self.map.len());
        println!("Translating: {} ", text);
        self.translator.translate(text).unwrap_or(DEFAULT_VALUE)
    }
}

// 这里不想要使用 clone 的 map, 我们在这里面只需要读取 map 
// 因此可以使用裸指针
struct PtrTranslate {
    // map: HashMap<String, String>,
    map: *const HashMap<String, String>,
}

impl PtrTranslate {

    fn translate(&self, text: &str) -> Option<&str> {
        // self.map.get(text).map(|s| s.as_str())

        // 裸指针访问需要借助 unsafe 解引用
        unsafe { &*self.map }.get(text).map(|s| s.as_str())
    }
}
