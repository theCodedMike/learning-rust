use unicode_segmentation::UnicodeSegmentation;
///
/// cargo r --bin usg
///
fn main() {
    assert_eq!(reverse_common_str(""), "".to_string());
    assert_eq!(reverse_common_str("robot"), "tobor".to_string());
    assert_eq!(reverse_common_str("Ramen"), "nemaR".to_string());
    assert_eq!(reverse_common_str("I'm hungry!"), "!yrgnuh m'I".to_string());
    assert_eq!(reverse_common_str("子猫"), "猫子".to_string());

    // left: "u\u{308}uu"  right: "uu\u{308}u"
    //assert_eq!(reverse_common_str("uüu"), "uüu".to_string());
    let res = reverse_uncommon_str("uaüu");
    println!("{}", res); // uüau
    let res = reverse_uncommon_str("子猫");
    println!("{}", res); // 猫子
    let res = reverse_uncommon_str("Ramen");
    println!("{}", res); // nemaR
    let res = reverse_uncommon_str("");
    println!("{}", res);
}

pub fn reverse_common_str(input: &str) -> String {
    let mut res = String::new();

    for char in input.chars().rev() {
        res.push(char)
    }

    res
}

pub fn reverse_uncommon_str(input: &str) -> String {
    let mut res = String::new();

    for char in input.graphemes(true).rev() {
        res.push_str(char);
    }

    res
}
