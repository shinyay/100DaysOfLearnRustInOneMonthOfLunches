fn main() {
    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: i32 = my_vec.iter().sum();

    println!("{:?}", result);
}
