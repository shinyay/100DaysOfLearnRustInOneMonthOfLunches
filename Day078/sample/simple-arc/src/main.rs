use std::{sync::Arc, thread};

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let shared_data = Arc::new(data);

    let thread1_data = shared_data.clone();
    let thread1 = thread::spawn(move || {
        println!("[Thread1] {:?}", thread1_data);
    });

    let thread2_data = shared_data.clone();
    let thread2 = thread::spawn(move || {
        println!("[Thread2] {:?}", thread2_data);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
