fn main() {
    let some_vec: Vec<&str> = vec!["1", "2", "3.0", "four", "FIVE"];
    let mut my_vec = vec![];

    for index in 0..some_vec.len() {
        my_vec.push(
            some_vec
            .get(index)  // [Some("1"), Some("2"), Some("3.0"), Some("four"), Some("FIVE")]
            .and_then(|item| item.parse::<i32>().ok())  // [Some(1), Some(2), None, None, None]
            .and_then(|item| f64::try_from(item).ok())  // [Some(1.0), Some(2.0), None, None, None]
        );
    }
    println!("{:?}", my_vec);
}