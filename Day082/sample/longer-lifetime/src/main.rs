use std::{sync::Arc, thread};

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let shared_data = Arc::new(data.clone());

    let handle = thread::spawn(move || {
        println!("ThreadL {:?}", shared_data);
    });

    handle.join().unwrap();
    println!("Main Thread: {:?}", data);
}
