fn my_function_fnonce<F: FnOnce()>(f: F) {
    f();
}

fn my_function_fnmut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn my_function_fn<F: Fn()>(f: F) {
    f();
    f();
}

fn main() {
    
}