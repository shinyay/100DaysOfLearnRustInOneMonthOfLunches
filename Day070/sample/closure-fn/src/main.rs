fn main() {
    let default_value = 10;
    let my_closure = |x: u32| {
        println!("Variable: {}", x);
        default_value * x
    };

    let result = my_closure(1);
    assert_eq!(10, result);
    println!("Result: {:?}", result);

    let result = my_closure(5);
    assert_eq!(50, result);
    println!("Result: {:?}", result);

    let result = my_closure(10);
    assert_eq!(100, result);
    println!("Result: {:?}", result);
}