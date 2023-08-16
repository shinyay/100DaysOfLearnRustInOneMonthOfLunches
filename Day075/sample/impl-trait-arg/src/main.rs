trait MyTrait {
    fn do_something(&self);
}

struct MyStruct;

impl MyTrait for MyStruct {
    fn do_something(&self) {
        println!("Doing something!!!");
    }
}

fn process_my_trait_impl(item: impl MyTrait) {
    item.do_something();
}

fn main() {
    let my_struct = MyStruct;
    process_my_trait_impl(my_struct);
}
