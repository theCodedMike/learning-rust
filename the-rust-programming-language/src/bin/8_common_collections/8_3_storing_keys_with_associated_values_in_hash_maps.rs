#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash64;

/// 8.3 在HashMap中存储 key 和 value
///
/// cargo r --bin hashmap
///
/// ## 目录
/// ### 新建一个哈希 map
///
/// ### 哈希 map 和所有权
///
/// ### 访问哈希 map 中的值
///
/// ### 更新哈希 map
/// #### 覆盖一个值
/// #### 只在键没有对应值时插入
/// #### 根据旧值更新一个值
///
/// ### 哈希函数
///   HashMap默认使用一种叫做SipHash的哈希函数(安全性最好, 性能不是最好的)
///   如果追求性能，可以使用三方库 twox-hash
///   f32 和 f64 浮点数，没有实现 std::cmp::Eq 特征，因此不可以用作 HashMap 的 Key
///
fn main() {
    /* 新建一个哈希 map */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let _map: HashMap<usize, usize> = HashMap::with_capacity(20);

    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);

    let arr_into_map: HashMap<i32, &str> = [(1, "hello"), (2, "two"), (3, "third")].into();

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("map is {:?}", scores); // {"Blue": 10, "Yellow": 50}
    println!();

    /* 哈希 map 和所有权 */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 这里发生了所有权转移, field_name 和 field_value 不再有效
                                         //println!("key is {}", field_name); // borrow of moved value: `field_name`

    /* 访问哈希 map 中的值 */
    let team_name = String::from("Blue");
    // 注意这里key必须是一个不可变引用
    if let Some(score) = scores.get(&team_name) {
        println!("value is {}", score); // 10
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
        /*
        Blue: 10
        Yellow: 50
         */
    }

    // let key = "hello".to_string();
    let x = scores[&team_name]; // 这种方式key必须是有效的, 否则会panic
    println!("x is {}", x); // x is 10
    println!();

    /* 更新哈希 map */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); // Blue -> 10 不会保留
    scores.insert(String::from("Blue"), 25); // Blue -> 25 覆盖前一个值

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); // Blue -> 10 会保留
    scores.entry(String::from("Blue")).or_insert(50); // 只在键没有对应值时插入 Blue -> 10

    println!("remove: {:?}", scores.remove("Blue")); // remove: Some(10)

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // 统计单词个数 - 解法1
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map); // {"world": 2, "wonderful": 1, "hello": 1}
    map.clear();
    // 统计单词个数 - 解法2
    for word in text.split_whitespace() {
        map.entry(word).and_modify(|v| *v += 1).or_insert(1);
    }
    println!("{:?}", map); // {"world": 2, "wonderful": 1, "hello": 1}
    println!();

    /* 哈希函数 */
    let mut map: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    map.insert("hello", 5);
    println!("{}", map["hello"]); // 5
    println!();

    /* 补充: HashMap扩容规律 */
    test_hashmap_cap_allocate();
}

/// Hashmap的动态内存扩展规律
///
/// 3 -> 7 -> 14 -> 28 -> 56 -> 112 ...
fn test_hashmap_cap_allocate() {
    let mut map = HashMap::new();
    for i in 1..=200 {
        map.insert(i, i.to_string());
        println!(
            "{:3} inserted, and map size: {:3}, cap: {:3}",
            i,
            map.len(),
            map.capacity()
        );
    }
}
