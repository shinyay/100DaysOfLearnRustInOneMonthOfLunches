use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(10);
    let mut mutex_changer = my_mutex.lock().unwrap();

    println!("{:?}", mutex_changer);
}
