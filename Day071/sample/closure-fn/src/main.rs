fn my_function_fn<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn main() {
    let my_vec = vec![1, 2, 3, 4, 5];
    my_function_fn(|| {
        my_vec
            .iter()
            .for_each(|x| println!("Count: {x}"));
    });
    my_function_fn(|| {
        my_vec
            .iter()
            .for_each(|x| println!("Doubled: {}", x*2));
    });
}
 