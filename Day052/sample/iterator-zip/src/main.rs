fn main() {
    let even_odd_vec = vec!["even", "odd"];

    let my_vec = (0..10)
        .zip(even_odd_vec.into_iter().cycle())
        .collect::<Vec<(i32, &str)>>();

    println!("{:?}", my_vec);
}
