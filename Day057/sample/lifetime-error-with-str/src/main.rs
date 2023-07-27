#[derive(Debug)]
struct Person {
    name: &str,
    age: u32,
}
 
fn main() {
    let me = Person {
        name: "Yanagihara",
        age: 30,
    };
}