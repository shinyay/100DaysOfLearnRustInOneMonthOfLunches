fn main() {
    let a = [-1, 0, 1];
    let mut result = a.iter().take_while(|x| **x < 0);
    println!("{:?}", result.next());
    println!("{:?}", result.next());
}