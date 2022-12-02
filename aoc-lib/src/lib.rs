use std::{error::Error, fmt::Display, io::Read};

pub fn read_input() -> Result<String, Box<dyn Error>> {
    let mut file = std::fs::File::open("./input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    return Ok(content);
}

pub fn print_result(result_one: &dyn Display, result_two: &dyn Display) -> () {
    println!("Part 1: {}\nPart 2: {}", result_one, result_two);
}
