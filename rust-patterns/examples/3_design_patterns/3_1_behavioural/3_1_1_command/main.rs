//! ## 命令模式
//!
//! ### 描述
//! 命令模式的基本概念是，将动作分离为单独的对象，并且作为参数传递它们
//!
//! ### 优点
//!
//! ### 缺点
//!
//! ### 讨论
//! 如果我们的命令很小，可以定义成函数，或作为闭包传递，那么使用函数指针可能更好， 因为它不需要动态分发
//!
//! 如果我们的命令是个完整的结构， 有一堆函数和变量被分别定义为独立的模块，那么使用trait对象会更合适
//!
//! 在使用Fn trait对象时， 我们可以用和函数指针相同的方式创建和使用命令
//!
//! 静态分发可以提供更好的性能，而动态分发在我们组织应用程序时提供了灵活性
//!
//! ### 执行
//! cargo r --example 3_1_1
//!

use crate::fn_ptr_model::{
    add_field as fn_ptr_add_field, remove_field as fn_ptr_remove_field, Schema as FnPtrSchema,
};
use crate::fn_trait_object_model::{
    add_field as fn_trait_obj_add_field, remove_field as fn_trait_obj_remove_field,
    Schema as FnTraitObjSchema,
};
use crate::trait_object_model::{AddField, CreateTable, Schema as TraitObjSchema};

mod fn_ptr_model;
mod fn_trait_object_model;
mod trait_object_model;

fn main() {
    // 特征对象版本
    let mut schema = TraitObjSchema::new();
    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);
    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());

    // 函数指针版本
    let mut schema = FnPtrSchema::new();
    schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
    schema.add_migration(fn_ptr_add_field, fn_ptr_remove_field);
    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());

    // Fn特征对象版本
    let mut schema = FnTraitObjSchema::new();
    schema.add_migration(|| "create table", || "drop table");
    schema.add_migration(fn_trait_obj_add_field, fn_trait_obj_remove_field);
    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}
