fn main() {
    let mut my_vec = vec!["Hello", "Rust", "on", "Jul 23"].into_iter();
    let hello: Vec<_> = my_vec.by_ref().take(2).collect();

    println!("MSG: {:?}", hello);

    let date: Vec<_> = my_vec.collect();

    println!("DATE: {:?}", date);
}
