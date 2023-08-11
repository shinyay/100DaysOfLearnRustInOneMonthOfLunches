fn main() {
    let mut init_value = 10;
    let mut my_closure = |x: i32| {
        println!("Initial Value: {}", init_value);
        init_value = init_value + x
    };

    my_closure(10);
    my_closure(10);
    my_closure(10);
    println!("Result: {}", init_value);
    assert_eq!(40, init_value);
}