use std::sync::Mutex;

pub fn concurrency_state() {
    let m = Mutex::new(7);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}