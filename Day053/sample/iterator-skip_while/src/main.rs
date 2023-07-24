fn main() {
    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = my_vec.iter().skip_while(|x| **x < 5).collect::<Vec<_>>();

    println!("{:?}", result);
}
