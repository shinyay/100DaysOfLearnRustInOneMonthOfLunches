fn main() {
    let my_vec = [1, 2, 3, 4, 5];

    let sum = my_vec.iter()
        .cloned()
        .filter(|x| x % 2 == 0)
        .fold(0, |sum, i| sum + i);
    
    println!("Result without inspect: {sum}");
    
    let sum = my_vec.iter()
        .cloned()
        .inspect(|x| println!("Current item: {x}"))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("Current item through Filter: {x}"))
        .fold(0, |sum, i| sum + i);
    
        println!("Result with inspect: {sum}");

    let sum = my_vec.iter()
        .cloned()
        .inspect(|x| {
            match x % 2 {
                0 => println!("Even: {}", x),
                _ => println!("Odd: {}", x),
            }
            println!("Current item: {x}");
        })
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("Current item through Filter: {x}"))
        .fold(0, |sum, i| sum + i);
    
    println!("Result with inspect: {sum}");
}