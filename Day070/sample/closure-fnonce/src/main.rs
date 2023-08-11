fn main() {
    let x = 10;
    
    // Using the `move` keyword to capture `x` by value
    let closure = move || {
        println!("Closure: x = {}", x);
    };
    
    closure();  // Call the closure
    
    // Uncommenting the following line would result in a compilation error
    // closure();  // Attempting to call the closure again would fail
}
