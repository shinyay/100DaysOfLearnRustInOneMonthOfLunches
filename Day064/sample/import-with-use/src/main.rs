enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

fn select_month(month: &Month) {
    match month {
        Month::January => println!("It's Winter."),
        Month::February => println!("It's Winter."),
        Month::March => println!("It's Spring."),
        Month::April => println!("It's Spring."),
        Month::May => println!("It's Spring."),
        Month::June => println!("It's Summer."),
        Month::July => println!("It's Summer."),
        Month::August => println!("It's Summer."),
        Month::September => println!("It's Fall."),
        Month::October => println!("It's Fall."),
        Month::November => println!("It's Fall."),
        Month::December => println!("It's Winter."),
        _ => println!("Etc."),
    }
}

fn main() {
    let month = Month::January;
    select_month(&month);
}
