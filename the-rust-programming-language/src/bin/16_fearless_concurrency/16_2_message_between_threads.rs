#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// 16.2 使用消息传递在线程间通信
///
/// cargo r --bin message
///
/// “Do not communicate by sharing memory; instead, share memory by communicating.”
///
/// ## 目录:
/// - mpsc: 多个生产者，单个消费者（multiple producer, single consumer）
/// - mpmc: 多个生产者，多个消费者，可以考虑三方库   crossbeam-channel  flume
/// ### 通道与所有权转移
/// ### 发送多个值并观察接收者的等待
/// ### 通过克隆发送者来创建多个生产者
///
fn main() {
    //message_channel();

    // 通道与所有权转移
    //move_ownership();

    // 发送多个值并观察接收者的等待
    //send_many_msgs();

    // 通过克隆发送者来创建多个生产者
    //use_clone_to_create_multi_sender();
}
fn message_channel() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("hello");
        tx.send(msg).unwrap();
    });
    let msg = rx.recv().unwrap(); // 阻塞主线程执行直到从通道中接收一个值
    println!("Got: {}", msg);
}
fn move_ownership() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //println!("val is {}", val); // val的所有权被转移到send方法内
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
fn send_many_msgs() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
fn use_clone_to_create_multi_sender() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}
