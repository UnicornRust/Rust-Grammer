// 
// drop trait 属于预导入模块
// 实现 drop trait 可以在对象离开作用域自动执行,通常用于释放资源解决
// 

pub fn drop_occation() {
    drop_trait_occasion();
}


struct CustomSmartPoint {
    data: String,
}

// Drop trait 执行的时机
// 修改 drop 实现，直接打印数据，查看对应的 drop trait 在什么时候会执行
impl Drop for CustomSmartPoint {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPoint with data `{}`!", self.data);
    }
}

// 
//
// 测试 Drop trait 执行的时机
// 同一个作用域中, 从代码执行顺序的反顺序来执行 drop 
//
fn drop_trait_occasion() {
    {
        let _c = CustomSmartPoint {
            data: String::from("inner staff"),
        };

    }
    let _c = CustomSmartPoint {
        data: String::from("my staff"),
    };
    let _d = CustomSmartPoint {
        data: String::from("other staff"),
    };
    println!("CustomSmartPoint created!");
}
