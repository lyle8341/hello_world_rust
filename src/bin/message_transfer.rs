use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // mpsc: multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        //thread::sleep(Duration::from_secs(5));
        let val = String::from("hi");
        tx.send(val).unwrap(); // val所有权已移交
    });

    // recv 阻塞线程，直到有消息
    // try_recv 不会阻塞
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
