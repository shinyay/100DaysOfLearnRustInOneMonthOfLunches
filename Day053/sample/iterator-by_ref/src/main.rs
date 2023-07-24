fn main() {
    let mut my_vec = vec!["Hello", "Rust", "on", "Jul 23"].into_iter();
    let hello: Vec<_> = my_vec.by_ref().take(2).collect();
    // let hello: Vec<_> = my_vec.take(2).collect(); // error[E0382]: use of moved value: `my_vec`

    println!("MSG: {:?}", hello);

    let date: Vec<_> = my_vec.collect();

    println!("DATE: {:?}", date);
}
