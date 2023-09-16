fn main() {
    let my_box: Box<i32> = Box::new(1);
    let my_int: i32 = *my_box;
    println!("Hello, Box: {}", my_box);
    println!("Hello, Box: {}", *my_box);
    println!("Hello, Box: {}", my_int);
}
