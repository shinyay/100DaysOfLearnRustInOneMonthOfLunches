# Day 21

## Notes

### Control flow

- `if`

Many others are used in much the same way as programming languages. Parentheses are not needed in the description of conditions.

```rust
fn main() {
    let today = "Thursday";
    if today == "Sunday" {
        println!("It's holiday.");
    } else if today == "Friday" {
        println!("It's TGIF!");
    } else {
        println!("It's weekday");
    }
}
```

## Key Takeaways
