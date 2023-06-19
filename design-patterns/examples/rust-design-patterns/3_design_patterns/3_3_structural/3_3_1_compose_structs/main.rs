#![allow(unused)]
//! ## 组合模式
//!
//! ### 描述
//! 有时候一个很大的结构体会在借用的时候产生问题，将这个大结构体分解成更小的结构体，
//! 然后再把这些小结构组装成大的结构体，这样结构体中的每个部分都可以单独的借用。
//!
//! ### 优点
//! 可以让你挣脱借用检查器的限制
//!
//! ### 缺点
//! 需要更多的代码   
//! 有时更小的结构体没有明确的抽象意义，最终导致做出坏设计
//!
//! ### 讨论
//! 将功能分解成更小的单元是很多有名的软件设计原则中都赞同的，这一点与语言无关
//!
//! ### 执行
//! cargo r --example 3_3_1
//!

use crate::after_refactoring::{ConnectionString, Database as RDatabase, PoolSize, Timeout};
use crate::model::{print_database, Database};

mod after_refactoring;
mod model;

fn main() {
    let mut db = Database {
        connection_string: "initial string".to_string(),
        timeout: 30,
        pool_size: 100,
    };

    let connection_string = &mut db.connection_string;
    // Immutable borrow of `db` happens here
    model::print_database(&db);
    // Mutable borrow is used here
    // *connection_string = "new string".to_string();

    /* After refactoring */
    // Initialize the Database with the three structs
    let mut db = RDatabase {
        connection_string: ConnectionString("localhost".to_string()),
        timeout: Timeout(30),
        pool_size: PoolSize(100),
    };

    let connection_string = &mut db.connection_string;
    after_refactoring::print_database(connection_string.clone(), db.timeout, db.pool_size);
    *connection_string = ConnectionString("new string".to_string());
    after_refactoring::print_database(connection_string.clone(), db.timeout, db.pool_size);
}
