fn main() {
    let my_vec = [1, 2, 3, 4, 5];

    let sum = my_vec.iter()
        .cloned()
        .filter(|x| x % 2 == 0)
        .fold(0, |sum, i| sum + i);
    
    println!("{sum}");
    
    let sum = my_vec.iter()
        .cloned()
        .inspect(|x| println!("Current item: {x}"))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("Current item through Filter: {x}"))
        .fold(0, |sum, i| sum + i);
    
    println!("{sum}");
}
