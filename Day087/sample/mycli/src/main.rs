use std::env;
use rand::seq::SliceRandom;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        println!("Usage: comforting-words [name]");
        println!("Prints a comforting message along with an optional name.");
        println!("If a name is provided, the message will be addressed to the name.");
        return;
    }

    let comforting_words = [
        "You're doing great!",
        "Take a deep breath and relax.",
        "This too shall pass.",
        "You're stronger than you think.",
        "Sending you positive vibes.",
        "You're not alone in this.",
        "Believe in yourself.",
        "You've got this!",
        "Stay positive and keep going.",
        "You're capable of amazing things.",
    ];

    let name = args.get(1).cloned();

    let mut rng = rand::thread_rng();
    let random_message = comforting_words.choose(&mut rng).unwrap();

    match name {
        Some(name) => println!("{}: {}", name, random_message),
        None => println!("{}", random_message),
    }
}
