fn main() {
    let some_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
 
    println!("{}", some_numbers
        .iter()
        .fold(0, |total_so_far, next_number| total_so_far + next_number)
    );
}
