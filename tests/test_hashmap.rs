
use rs_grammer::variable::vhashmap;

// 用于构建测试子模块，保存测试需要用到的公用代码
//
mod common;

#[test]
fn test_hashmap() {
    common::setup();
    vhashmap::hashmap_type();
}
