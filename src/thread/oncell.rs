use std::cell::OnceCell;

pub fn run() {

    api();
    init_api();
    mut_api();
}

fn api() {

    let cell = OnceCell::new();
    // 
    // get -> Option<T> (初始化之后得到的是一个 None)
    assert!(cell.get().is_none());

    // set -> Result<T, E> (第一个设置应该成功)
    let  result = cell.set(String::from("hello"));
    assert!(result.is_ok());

    // 再次设置应该不成功
    let result = cell.set(String::from("world"));
    assert!(result.is_err());

}

fn init_api() {

    let cell = OnceCell::new();
    // 
    // get -> Option<T> (初始化之后得到的是一个 None)
    assert!(cell.get().is_none());

    // get_or_init(fn)  获取或者初始化之后获取
    let value = cell.get_or_init(|| {
        "hello world".to_string()
    });
    assert_eq!(value, "hello world");
    assert!(cell.get().is_some());
}


// get_mut() 获取内部值的可变引用，但只有你对OnceCell 本身持有一个可变引用时才能使用

fn mut_api() {

    let mut cell = OnceCell::new();
    
    let _ = cell.set(String::from("hello"));

    // 当本身持有一个可变引用时则可通过可变引用对内部值进行修改
    if let Some(mut_cell) = cell.get_mut() {
        *mut_cell = String::from("final");
    };

    // 再次调用 set 必然会时候
    let result = cell.set(String::from("peek"));
    assert!(result.is_err());

    if let Some(value) = cell.get() {
        assert_eq!(value, "final");
    }
}
