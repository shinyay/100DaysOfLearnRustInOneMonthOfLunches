use std::fmt::Display;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl Person<'_> {
    fn say_hello(&self) {
        println!("{} says Hello", self.name);
    }
}

impl Display for Person<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}) says Hello", self.name, self.age)
    }
}
fn main() {
    let me = Person {
        name: "Shinya",
        age: 40,
    };

    println!("{:?}", me);
    dbg!(&me);
    println!("{}", me);
    me.say_hello();
}
