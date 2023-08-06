type MyCharVec = Vec<char>;

fn main() {
    let mut my_chars: MyCharVec = Vec::new();
    
    // my_chars.push(1); // error[E0308]: mismatched types - expected `char`, found `u8`
    // my_chars.push(2);
    // my_chars.push(3);

    my_chars.push('A');
    my_chars.push('B');
    my_chars.push('C');

    println!("MyCharVec: {:?}", my_chars);
}