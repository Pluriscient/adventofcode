use std::fs::read_to_string;

pub fn read_input(day: usize) -> std::io::Result<String> {
    read_to_string(format!("inputs/year{}/day{}.txt", 2021, day))
}


pub fn read_input_test(day: usize) -> std::io::Result<String> {
    read_to_string(format!("inputs/year{}/day{}.test", 2021, day))
}

// Change the alias to `Box<error::Error>`.
pub type AResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;