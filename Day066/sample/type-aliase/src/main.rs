type MyCharVec = Vec<char>;

fn main() {
    let mut my_chars: MyCharVec = Vec::new();
    
    my_chars.push(1); // error[E0308]: mismatched types - expected `char`, found `u8`

    println!("MyCharVec: {:?}", my_chars);
}