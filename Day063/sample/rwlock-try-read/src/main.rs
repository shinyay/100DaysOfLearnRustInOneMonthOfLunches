use std::sync::RwLock;
 
fn main() {
    let my_rwlock = RwLock::new(1);

    let my_read = my_rwlock.read().unwrap();
    println!("The value from RwLock: {}", my_read);

    drop(my_read);

    if let Ok(mut value) = my_rwlock.try_write() {
        *value += 10;
        println!("The value from RwLock: {}", value);
    } else {
        println!("Can't Get Write Access")
    };
}