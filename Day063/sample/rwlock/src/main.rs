use std::sync::RwLock;
 
fn main() {
    let my_rwlock = RwLock::new(1);
 
    let my_read = my_rwlock.read().unwrap();
    println!("{:?}", my_read);
 
    // This program runs forever withoud `drop()`
    drop(my_read);

    let mut my_write = my_rwlock.write().unwrap();
    *my_write += 1;

    println!("{:?}", my_rwlock);
}