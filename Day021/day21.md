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

You can use a **turple** for `match`

```rust
fn match_colours(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Each colour has at least 10"),
    }
}
 
fn main() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
 
    match_colours(first);
    match_colours(second);
    match_colours(third);
 
}
```

- [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8d15b47faa592d1aafddca8be33cb607)

## Key Takeaways
