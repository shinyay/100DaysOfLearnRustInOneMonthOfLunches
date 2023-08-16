trait MyTrait {
    fn do_something(&self);
}

struct MyStruct;

impl MyTrait for MyStruct {
    fn do_something(&self) {
        println!("Doing Something...");
    }    
}

fn get_my_trait_impl() -> impl MyTrait {
    MyStruct
}

fn main() {
    let my_trait_impl = get_my_trait_impl();
    my_trait_impl.do_something();
}