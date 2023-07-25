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

    println!("Window size: 5");
    for iter in my_slice.windows(5) {
        println!("{:?}", iter);
    }
}
