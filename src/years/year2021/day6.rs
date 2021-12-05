
trait AOCDay {
    type Input;
    fn part_one(&mut self, input: Input)
    fn part_two(&mut self, input: Input)
    fn parse_input(&mut self, input: &str) -> Self::Input;
}

macro_rules! day {
    ($day:ident, $year:expr, $input:expr) => {
        #[allow(dead_code)]
        pub mod $day {
            use super::*;
            use std::io::{self, BufRead};
            use std::collections::HashMap;
            use std::collections::HashSet;

        }
    };
}
