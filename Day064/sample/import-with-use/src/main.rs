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

fn select_month_refined(month: &Month) {
    use Month::*;

    let winter = "Winter";
    let spring = "Spring";
    let summer = "Summer";
    let fall = "Fall";

    match month {
        January => println!("{winter}"),
        February => println!("{winter}"),
        March => println!("{spring}"),
        _ => println!("Etc."),
        
    }
}

fn select_month_abbreviation(month: &Month) {
    use Month::{
        January as JAN,
        February as FEB,
        March as MAR,
        April as APR,
        May as MAY,
        June as JUN,
        July as JUL,
        August as AUG,
        September as SEP,
        October as OCT,
        November as NOV,
        December as DEC,
    };

    match month {
        JAN => println!("It's Winter."),
        FEB => println!("It's Winter."),
        MAR => println!("It's Spring."),
        APR => println!("It's Spring."),
        MAY => println!("It's Spring."),
        JUN => println!("It's Summer."),
        JUL => println!("It's Summer."),
        AUG => println!("It's Summer."),
        SEP => println!("It's Fall."),
        OCT => println!("It's Fall."),
        NOV => println!("It's Fall."),
        DEC => println!("It's Winter."),
        _ => println!("Etc."),
    }
}

fn main() {
    let month = Month::January;
    select_month(&month);

    let month = Month::March;
    select_month_refined(&month);
}
