#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// 16.2 使用消息传递(message passing)在线程间通信
///
/// cargo r --bin message
///
/// “Do not communicate by sharing memory; instead, share memory by communicating.”
///
/// ## 目录
/// - 为了实现消息传递并发，Rust 标准库提供了一个 信道(channel)实现
/// - 信道是一个通用编程概念, 表示数据从一个线程发送到另一个线程
/// - 编程中的信息渠道(信道)有两部分组成，一个发送者(transmitter)和一个接收者(receiver)
/// - 当发送者或接收者任一被丢弃时可以认为信道被 关闭(closed)了
/// - mpsc: 多个生产者，单个消费者(multiple producer, single consumer)
/// - mpmc: 多个生产者，多个消费者，可以考虑三方库   crossbeam-channel  flume
/// ### 通道与所有权转移
///
/// ### 发送多个值并观察接收者的等待
///
/// ### 通过克隆发送者来创建多个生产者
///
fn main() {
    //message_channel();

    /* 通道与所有权转移 */
    //move_ownership();

    /* 发送多个值并观察接收者的等待 */
    //send_many_msgs();

    /* 通过克隆发送者来创建多个生产者 */
    //use_clone_to_create_multi_sender();

    /* 补充 */
    //send_and_recv_each_other();

    // 同步发送消息
    //sync_send();

    // 一个小坑
    caution();
}

///
/// 1 sender, 1 receiver
///
/// 子线程 向 主线程/子线程 发送1条消息
///
/// 如果想同时发送多种类型的消息，可以使用枚举包装一下，或者使用特征对象
///
fn message_channel() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("hello");
        match tx.send(msg) {
            Ok(_) => {
                println!("child thread successfully sent one msg")
            }
            Err(e) => {
                println!("child thread failed to send msg: {}", e);
            }
        }
    });
    let recv_in_main_thread = false;

    if recv_in_main_thread {
        let msg = rx.recv().unwrap(); // 阻塞主线程执行直到从通道中接收一个值
                                      //rx.try_recv(); // try_recv()不会阻塞
        println!("main thread got one msg: {}", msg);
    } else {
        thread::spawn(move || {
            let msg = rx.recv().unwrap();
            println!("child thread got one msg: {}", msg);
        })
        .join()
        .unwrap();
    }
    // child thread successfully sent one msg
    // child thread got one msg: hello
}

fn move_ownership() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("send() owned the ownership of val");
        tx.send(val).unwrap();
        //println!("val is {}", val); // val的所有权被转移到send方法内
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    println!();
}

///
/// 1 sender, 1 receiver  
///
/// 子线程 向 主线程 发送多条消息
///
fn send_many_msgs() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("child"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            //thread::sleep(Duration::from_secs(1));
        }
        println!("child thread sent many msgs");
    });

    let use_for_to_recv = false;
    if use_for_to_recv {
        for received in rx {
            // no recv(), but main thread still wait to receive msg from child thread
            println!("main thread got: {}", received);
        }
    } else {
        rx.into_iter()
            .for_each(|msg| println!("main thread got: {}", msg));
    }
}

///
/// 2 sender, 1 receiver
///
/// 2个子线程向主线程发送多条消息
///
fn use_clone_to_create_multi_sender() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("1 hi"),
            String::from("1 from"),
            String::from("1 the"),
            String::from("1 child"),
            String::from("1 thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("2 more"),
            String::from("2 messages"),
            String::from("2 for"),
            String::from("2 you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            //thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}

///
/// 2个子线程互相发送1条消息
///
fn send_and_recv_each_other() {
    let (sender1, recver1) = mpsc::channel();
    let (sender2, recver2) = mpsc::channel();
    thread::spawn(move || {
        sender1.send("hello from sender1").unwrap();
        drop(sender1);
        //thread::sleep(Duration::from_secs(2));
        println!("thread1 recv: {}", recver2.recv().unwrap());
    });
    thread::spawn(move || {
        println!("thread2 recv: {}", recver1.recv().unwrap());
        drop(recver1);
        sender2.send("hello from sender2").unwrap();
    })
    .join()
    .unwrap();
    // thread2 recv: hello from sender1
    // thread1 recv: hello from sender2
}

///
/// 同步发送消息
///
/// This channel has an internal buffer on which messages will be queued. bound specifies the buffer size.
///
fn sync_send() {
    let (sender, receiver) = mpsc::sync_channel(2);
    // this returns immediately
    sender.send(1).unwrap();
    thread::spawn(move || {
        // this will block until the previous message has been received
        sender.send(2).unwrap();
    });

    assert_eq!(receiver.recv().unwrap(), 1);
    assert_eq!(receiver.recv().unwrap(), 2);
}

///
/// recv will block until a message is available while there is at least one Sender alive (including clones).
///
/// 只要有1个sender存活，recv方法会一直阻塞主线程
///
fn caution() {
    let (sender, recver) = mpsc::channel();
    let num_threads = 5;
    for i in 0..num_threads {
        let sender_clone = sender.clone();
        thread::spawn(move || {
            sender_clone.send(i).unwrap();
            println!("thread {} finished", i);
        });
    }
    // 注意这里需要 drop sender，否则主线程会一直被阻塞，因为如果sender不释放，channel就不会被关闭，所以recver需要一直等待以接收消息
    drop(sender);
    for x in recver {
        println!("Got: {}", x);
    }
}
