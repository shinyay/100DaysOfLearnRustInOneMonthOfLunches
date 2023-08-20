use std::{sync::Arc, thread};

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    let cloned_data = data.clone();

    let thread1 = thread::spawn(move || {
        for value in cloned_data.iter() {
            println!("[Thread1: doubled-data] {:?}", value * 2);
        }
    });

    let cloned_data = data.clone();

    let thread2 = thread::spawn(move || {
        for value in cloned_data.iter() {
            println!("[Thread2: tripled-data] {:?}", value * 3);
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
