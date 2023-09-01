use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Shinya Yanagihara",
    version = "0.0.1",
    about = "My first cli with clap"
)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}