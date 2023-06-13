fn main() {
    let string1 = "to_string".to_string();
    println!("{}", string1);
    
    let string2 = "to_owned".to_owned();
    println!("{}", string2);
    
    let string3 = String::from("String::from");
    println!("{}", string3);
    
    let string4 = "format!()";
    println!("{}", format!("{}, This is Macro.", string4));
}