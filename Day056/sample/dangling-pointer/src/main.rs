fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &'static String {
    let my_string = String::from("Hello");

    &my_string
}