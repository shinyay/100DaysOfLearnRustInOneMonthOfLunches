# Day 21

## Notes

### Control flow

- `if`

Many others are used in much the same way as programming languages. Parentheses are not needed in the description of conditions.

```rust
fn main() {
    let today = "Thursday";
    if today == "Sunday" || today == "Saturday" {
        println!("It's holiday.");
    } else if today == "Friday" {
        println!("It's TGIF!");
    } else {
        println!("It's weekday");
    }
}
```

You can use `match` instead of `if`.

- `match`: put the name of the item to match against, and then a `{}` code block
- `=>`: fat arrow to say what to do when it matches
  - the pattern on the left of the fat arrow
- **arm**: Each line is called
- put a comma between the arms (not a semicolon)

```rust
fn main() {
    let today: u8 = 30;
    match today {
        0 => println!("No such day."),
        1 => println!("it's the first day"),
        15 => println!("it's the middle of the month."),
        30 => println!("it's the end of the month."),
        _ => println!("It's some other day"),
    }
}
```

You can declare a value with `match`

```rust
fn main() {
    let today: u8 = 30;
    let msg = match today {
        0 => "No such day.",
        1 => "it's the first day",
        15 => "it's the middle of the month.",
        30 => "it's the end of the month.",
        _ => "It's some other day",
    };
    println!("{msg}");
}
```

## Key Takeaways
