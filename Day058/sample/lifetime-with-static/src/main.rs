#[derive(Debug)]
struct Person {
    name: & 'static str,
    age: u32,
}
 
fn main() {
    let people_vec = vec!["Yanagihara", "John"];

    let me = Person {
        name: people_vec[0],
        age: 30,
    };

    println!("{:?}", me);
}