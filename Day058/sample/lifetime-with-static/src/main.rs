// #[derive(Debug)]
// struct Person {
//     name: & 'static str,
//     age: u32,
// }
 
// fn main() {
//     let people_vec = vec!["Yanagihara".to_string(), "John".to_string()];

//     let me = Person {
//         name: &people_vec[0],
//         age: 30,
//     };

//     println!("{:?}", me);
// }

#[derive(Debug)]
struct Person<'a> {
    name: & 'a str,
    age: u32,
}
 
fn main() {
    let people_vec = vec!["Yanagihara".to_string(), "John".to_string()];

    let me = Person {
        name: &people_vec[0],
        age: 30,
    };

    println!("{:?}", me);
}