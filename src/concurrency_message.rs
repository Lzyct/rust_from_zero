use std::{sync::mpsc, thread, time::Duration};

pub fn concurrency_message() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let hi = String::from("Hi, Lzyct");
        sender.send(hi).unwrap();
    });

    let received = receiver.recv().unwrap();
    println!("Got: {}", received);

    let (sender1, receiver1) = mpsc::channel();

    let sender1cp = sender1.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("Lzyct"),
        ];

        for val in vals {
            sender1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("Lzyct"),
        ];

        for val in vals {
            sender1cp.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in receiver1 {
        println!("Got2: {}", received);
    }
}