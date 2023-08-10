fn main() {
    let mut my_vec = vec![];
    for count in 0..10 {
        my_vec.push(
          std::thread::spawn(move || {
            println!("[{}] Hello, thread!", count);
          })  
        );
    }

    for handle in my_vec {
        handle.join().unwrap();
    }
}