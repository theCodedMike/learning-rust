use crate::default_trait::MyConfiguration;

mod default_trait;

//! ## Default特征
//!
//! ### 描述
//!
//! ### 执行
//! cargo r --example 2_4
//!

fn main() {
    let conf = MyConfiguration::default();
    println!("conf = {:#?}", conf);
    // conf = MyConfiguration {
    //     output: None,
    //     search_path: [],
    //     timeout: 0ns,
    //     check: false,
    //     age: 0,
    //     height: 0.0,
    //     ch: '\0',
    //     name: "",
    // }
    println!("{:?}", conf.get_output());
    println!("{:?}", conf.get_search_path());
    println!("{:?}", conf.get_timeout());
    println!("{}", conf.get_check());
    println!("{}", conf.get_ch());
    println!("{}", conf.get_name());
    println!("{}", conf.get_age());
    println!("{}", conf.get_height());
}
