use std::fs::read_to_string;
use std::io;

pub fn read_input(day: usize) -> std::io::Result<String> {
    read_to_string(format!("inputs/year{}/day{}.txt", 2020, day))
}