fn my_function_fnonce<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn main() {
    let my_vec = vec![1, 2, 3, 4, 5];
    my_function_fnonce(|| {
        my_vec
            .into_iter()
            .for_each(|x| println!("Count: {x}"));
    });
    // my_function_fnonce(|| {
    //     my_vec
    //         .into_iter()
    //         .for_each(|x| println!("Count: {x}"));
    // });
}
 