fn main() {
    for count in 1..10 {
        std::thread::spawn(move || {
            println!("[{count}] Hello, world!");
        });
    }
}
