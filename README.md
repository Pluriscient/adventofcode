# AOC 2020

My work on the [Advent of Code 2020](https://adventofcode.com/).
Unfortunately I don't think I'll have time to complete it all but we'll see how far we can get.

Primary language: Rust

## Live template
For starting a day I'm using a live template in CLion, feel free to use it as well:

```rust
fn solve_part_one($arg$: $type$) -> $res$ {
    todo!()
}

fn solve_part_two($arg$: $type$) -> $res$ {
    todo!()
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;
    use super::{solve_part_one, solve_part_two};
    
    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = read_to_string("inputs/$file$.txt")?;
        let lines = input.split_ascii_whitespace();
        // todo transform lines to our input somehow
        let $arg$ = todo!();
        let solution = solve_part_one($arg$);
        println!("solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Error> {
        let input = read_to_string("inputs/$file$.txt")?;
        let lines = input.split_ascii_whitespace();
        // todo transform lines to our input somehow
        let $arg$ = todo!();
        let solution = solve_part_two($arg$);
        println!("solution part 2: {}", solution);
        Ok(())
    }

}

```
