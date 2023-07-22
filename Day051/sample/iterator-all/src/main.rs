fn main() {
    let my_vec: Vec<i32> = vec![1, 2, 3];
    println!("Negative number is included?:  {}", my_vec.iter().all(|&item| item.is_negative()));

    let my_vec: Vec<i32> = vec![-1, 0, 1, 2, 3];
    println!("Negative number is included?:  {}", my_vec.iter().all(|&item| item.is_negative()));

    let my_vec: Vec<i32> = vec![-1, -2, -3];
    println!("Negative number is included?:  {}", my_vec.iter().all(|&item| item.is_negative()));
}