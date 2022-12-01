#![allow(unused)]

use std::io::Read;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("a: {}", one()?);
    println!("b: {}", two()?);
    Ok(())
}

fn one() -> std::result::Result<i32, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open("./input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content);

    return Ok(content
        .split("\n\n")
        .map(|elf| {
            elf.split_terminator("\n")
                .fold(0, |acc, line| acc + line.parse::<i32>().unwrap())
        })
        .max()
        .unwrap());
}

fn two() -> std::result::Result<i32, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open("./input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content);

    let mut elves: Vec<i32> = content
        .split("\n\n")
        .map(|elf| {
            elf.split_terminator("\n")
                .fold(0, |acc, line| acc + line.parse::<i32>().unwrap())
        })
        .collect();

    elves.sort_unstable();
    return Ok(elves[elves.len() - 3..].iter().sum::<i32>());
}
