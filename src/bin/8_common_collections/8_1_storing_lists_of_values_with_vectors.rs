
/// 8.1 使用vector存储一系列值
/// cargo r --bin 8_1
fn main() {
    /*
    ## vector 用来储存一系列的值
    ### 新建 vector
    ### 更新 vector
    ### 丢弃 vector 时也会丢弃其所有元素
    ### 读取 vector 的元素
    ### 遍历 vector 中的元素
    ### 使用枚举来储存多种类型
     */
    // 新建 vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // 更新 vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 丢弃 vector 时也会丢弃其所有元素
    {
        let v = vec![1, 2, 3, 4];

        // 处理变量 v

    } // <- 这里 v 离开作用域并被丢弃

    // 读取 vector 的元素
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // 索引
    println!("The third element is {}", third);
    match v.get(2) { // get方法
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    let does_not_exist = &v[100]; // panic
    let does_not_exist = v.get(100); // 不会panic，值为None

    // 遍历 vector 中的元素
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // 使用枚举来储存多种类型
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}