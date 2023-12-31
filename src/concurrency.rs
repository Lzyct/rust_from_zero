use std::{thread, time::Duration};

pub fn concurrency() {
    let handler = thread::spawn(|| {
        for i in 1..11 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handler.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];
    let handle1 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle1.join().unwrap();
}