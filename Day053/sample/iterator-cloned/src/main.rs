fn main() {
    let my_vec = vec![1, 2, 3, 4, 5];

    let cloned_vec = my_vec.iter().cloned().collect::<Vec<_>>();
    let mapped_vec = my_vec.iter().map(|x| x).collect::<Vec<_>>();
    let clone_vec = my_vec.iter().map(|x| x.clone()).collect::<Vec<_>>();

    println!("{:?}", cloned_vec);
    println!("{:?}", mapped_vec);
    println!("{:?}", clone_vec);
    assert_eq!(cloned_vec, my_vec);
}
