#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

/// 8.3 在HashMap中存储 key 和 value
/// cargo r --bin 8_3
fn main() {
    /*
    ## 哈希 map 储存键值对
    ### 新建一个哈希 map
    ### 哈希 map 和所有权
    ### 访问哈希 map 中的值
    ### 更新哈希 map
    #### 覆盖一个值
    #### 只在键没有对应值时插入
    #### 根据旧值更新一个值
    ### 哈希函数

    Rust's collections can be grouped into four major categories:
    - Sequences: Vec, VecDeque, LinkedList
    - Maps: HashMap, BTreeMap
    - Sets: HashSet, BTreeSet
    - Misc: BinaryHeap
     */
    // 新建一个哈希 map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("map is {:?}", scores); // {"Blue": 10, "Yellow": 50}

    // 哈希 map 和所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 这里发生了所有权转移
                                         // 这里 field_name 和 field_value 不再有效，
                                         // 尝试使用它们看看会出现什么编译错误！
                                         //println!("key is {}", field_name); // borrow of moved value: `field_name`

    // 访问哈希 map 中的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("value is {}", score.expect("value not exist")); // 10
    for (key, value) in &scores {
        println!("{}: {}", key, value);
        /*
        Blue: 10
        Yellow: 50
         */
    }

    // 更新哈希 map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); // Blue -> 10 不会保留
    scores.insert(String::from("Blue"), 25); // 覆盖一个值

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); // Blue -> 10 会保留
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // 只在键没有对应值时插入

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map); // {"wonderful": 1, "hello": 1, "world": 2}
}
