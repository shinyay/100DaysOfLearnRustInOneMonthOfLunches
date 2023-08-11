fn my_function_fnonce<F: FnOnce()>(f: F) {
    print!("[my_function_fnonce]");
    f();
}

fn my_function_fnmut<F: FnMut()>(mut f: F) {
    print!("[my_function_fnmut]");
    f();
    print!("[my_function_fnmut]");
    f();
}

fn my_function_fn<F: Fn()>(f: F) {
    print!("[my_function_fn]");
    f();
    print!("[my_function_fn]");
    f();
}

fn main() {
    let mut my_string = "Hello Closure".to_string();
    
    let my_closure_fn = || {
        println!("{}", my_string);
    };
    my_closure_fn();
    my_function_fn(my_closure_fn);

    let mut my_closure_fnmut = || {
        my_string.push_str(" in Rust");
        println!("{}", my_string);
    };
    my_closure_fnmut();
    my_function_fnmut(my_closure_fnmut);

    let my_closure_drop = || {
        println!("Dropping: {}", my_string);
        drop(my_string);
    };
    // my_closure_drop();
    my_function_fnonce(my_closure_drop);
}