fn main() {
    let my_vec = vec![1, 2, 3, 4, 5];

    let cloned_vec = my_vec.iter().cloned().collect::<Vec<_>>();
    let mapped_vec = my_vec.iter().map(|x| x).collect::<Vec<_>>();

    println!("{:?}", cloned_vec);
    println!("{:?}", mapped_vec);

}
