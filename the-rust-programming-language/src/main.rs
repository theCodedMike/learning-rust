#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

fn main() {
    println!("hello world");
    //test_vec_cap_allocate();
    test_hashmap_cap_allocate();
}
/// Vec的动态内存扩展规律
///
/// 4 -> 8-> 16 -> 32 -> 64 ...
fn test_vec_cap_allocate() {
    let mut vec = Vec::new();
    for i in 1..=50 {
        vec.push(i);
        println!(
            "{} inserted, and vec size: {}, cap: {}",
            i,
            vec.len(),
            vec.capacity()
        );
    }
}
/// Hashmap的动态内存扩展规律
///
/// 3 -> 7 -> 14 -> 28 -> 56 -> 112 ...
fn test_hashmap_cap_allocate() {
    let mut map = HashMap::new();
    for i in 1..=200 {
        map.insert(i, i.to_string());
        println!(
            "{} inserted, and map size: {}, cap: {}",
            i,
            map.len(),
            map.capacity()
        );
    }
}
