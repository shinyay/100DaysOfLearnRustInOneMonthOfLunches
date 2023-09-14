extern crate chrono;

use chrono::prelude::*;

fn main() {
    let local: DateTime<Local> = Local::now();
    println!("Hello, world! Current date and time: {}", local);
}
