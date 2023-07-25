fn main() {
    let my_slice = ['H', 'e', 'l', 'l', 'o', 'R', 'u', 's', 't', '!'];
    println!("Chunk size: 3");
    for chunk in my_slice.chunks(3) {
        println!("{:?}", chunk);
    }
    println!("Chunk size: 5");
    for chunk in my_slice.chunks(5) {
        println!("{:?}", chunk);
    }
}
