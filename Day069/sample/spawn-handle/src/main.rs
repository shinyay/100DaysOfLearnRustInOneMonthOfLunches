fn main() {
    for count in 1..10 {
        let my_hundle = std::thread::spawn(move || {
            println!("[{}] Hello, thread!", count)
        });

        my_hundle.join();
    }
}
