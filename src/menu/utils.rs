use std::io;
use std::num::ParseIntError;

pub fn read_integer() -> Result<i32, ParseIntError> {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read from stdin.");

    let trimmed = input_text.trim();
    return trimmed.parse::<i32>();
}
