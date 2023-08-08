use std::rc::Rc;

fn main() {
    let my_string: Rc<String> = Rc::new("Hello Rc in the Rust".to_string());
    let cloned_string1 = my_string.clone();
    let cloned_string2 = my_string.clone();

    println!("my_string: {:p}", &*my_string);
    println!("cloned_string1: {:p}", &*cloned_string1);
    println!("cloned_string2: {:p}", &*cloned_string2);
}
