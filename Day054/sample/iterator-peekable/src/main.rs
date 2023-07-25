fn main() {
    let mut my_iter = [1, 2, 3, 4, 5].iter().peekable();

    for _ in 0..6 {
        println!("{:?}", my_iter.peek());
        println!("{}", my_iter.peek().unwrap());
        my_iter.next();
    }
}
