use std::collections::HashMap;

pub mod metype;

// 代码块 unsafe {}
// 指针 (*const T  / *mut T)


const DEFAULT_VALUE: &str = "";

struct MySelfRefEngine {
    map: HashMap<String, String>,
    translator: PtrTranslate,
}

impl MySelfRefEngine {
    pub fn new(map: HashMap<String, String>) -> Self {
        Self {
            map: map.clone(),
            translator: PtrTranslate { map },
        }
    }
    fn translate(&self, text: &str) -> &str {
        print!("Translating: {} ", text);
        self.translator.translate(text).unwrap_or(DEFAULT_VALUE)
    }
}


struct PtrTranslate {
    map: HashMap<String, String>,
}

impl PtrTranslate {
    fn translate(&self, text: &str) -> Option<&str> {
        self.map.get(text).map(|s| s.as_str())
    }
}
