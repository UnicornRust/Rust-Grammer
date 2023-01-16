use std::collections::HashMap;

/// hashmap
pub fn hashmap_type() {
    println!("--------------------test of hashmap---------------");
    // hashmap 创建
    test_hash_map();
    // vec 迭代后转化成 hashmap
    vec_iter_to_hashmap();
    // 迭代 hashmap 元素
    iter_hashmap_item();
    // 判断是否存在 key，
    contain_item();
    // 根据 key 访问 item
    get_item();
    // update item
    update_item();
    // hashmap 的所有权
    ownership_hashmap();
}

fn test_hash_map() {
    let mut heroes = HashMap::new();
    heroes.insert("superman", "Clark Kent");
    heroes.insert("batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    if heroes.contains_key(&"batman") {
        let batman = heroes.get(&"batman");
        match batman {
            Some(x) => println!("{} is hero", x),
            None => println!(" No hero"),
        }
    }
    println!("heroes length is {}", heroes.len());
}

fn vec_iter_to_hashmap() {
    let terms = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = terms.into_iter().zip(initial_scores.into_iter()).collect();
    for (k, v) in scores.into_iter() {
        println!("{k}:{v}");
    }
}

fn iter_hashmap_item() {
    let mut heroes = HashMap::new();
    heroes.insert("superman", "Clark Kent");
    heroes.insert("batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    // 使用 vec 类似的方式来遍历
    for (key, val) in &heroes {
        println!("{key}:{val}");
    }
}

fn contain_item() {
    let mut heroes = HashMap::new();
    heroes.insert("superman", "Clark Kent");
    heroes.insert("batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    if heroes.contains_key(&"batman") {
        let batman = heroes.get(&"batman");
        match batman {
            Some(x) => println!("{} is hero", x),
            None => println!(" No hero"),
        }
    }
    println!("heroes length is {}", heroes.len());
}

fn get_item() {
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow"), 10);

    let team_name = String::from("Blue");

    // 获取到元素
    let score = map.get(&team_name);
    if let Some(x) = score {
        println!("{x}");
    }
}

/**
 * hashmap 在更新的时候有很多种的可能，因此也会产生多种的操作结果
 *  - 已经存在旧值，则覆盖旧值
 *  - 存在 key, 没有对应值时插入
 *  - 根据存在的键值对旧值做一些操作
 */

fn update_item() {
    // insert 插入同样的 key，不同的 value, 则覆盖旧值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("hashmap 新值覆盖结果: {:?}", scores);

    // entry 获取我们想要检查的 key 作为参数，返回一个
    // Entry, 它代表了可能存在也可能不存在值，
    // 我们可以利用这个结果进一步操作
    // .or_insert(val) 当 key 对应的val 不存在时设置val
    // 并返回值的可变引用，如果存在，直接返回对应值的可变引用
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(100);
    println!("hashmap 值空插入结果:{:?}", scores);

    // 如果存在旧值就对旧值做一些操作
    // 这里我们需要统计一段文本中每个单词出现的次数
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // split_whitespace 会以空白字符作为分隔符将一段
    // 文本分成一个 字符串 slice 列表
    // 然后使用 for 循环迭代遍历他们
    for word in text.split_whitespace() {
        // 如果值不存在则插入 0 并同时返回对应值的
        // 可变引用
        let count = map.entry(word).or_insert(0);
        // 要改变值，需要解引用
        // 如果值存在则在原来的值的基础上 +1
        *count += 1
    }
    println!("hashmap 变更原值结果: {:?}", map)
}

fn ownership_hashmap() {
    let field_name = String::from("Blue");
    let field_value = String::from("Yellow");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // 这里 field_name 和 field_value 所有权被移动到
    // haspmap 所有，不能再被访问
    //

    //  -------------------------------------------
    // 如果将值的 引用插入 hashmap, 这些值本身将不会
    // 被移动到 hashmap, 但是这些引用指向的值必须至少再hashmap
    // 有效时也是有效的
}
