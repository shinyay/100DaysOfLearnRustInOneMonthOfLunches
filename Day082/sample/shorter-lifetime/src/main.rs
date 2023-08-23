use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let referenced_data = &data;

    let handle = thread::spawn(move || {
        println!("Thread: {:?}", referenced_data);
    });

    handle.join().unwrap();
    println!("Main Thread: {:?}", data);
}
