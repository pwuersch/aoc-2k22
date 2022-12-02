#![allow(unused)]

use std::io::Read;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("a: {}", one()?);
    println!("b: {}", two()?);
    aoc_lib::print_result(&one()?, &two()?);
    Ok(())
}

fn one() -> std::result::Result<i32, Box<dyn std::error::Error>> {
    let mut input = aoc_lib::read_input()?;

    return Ok(input
        .split("\n\n")
        .map(|elf| {
            elf.split_terminator("\n")
                .fold(0, |acc, line| acc + line.parse::<i32>().unwrap())
        })
        .max()
        .unwrap());
}

fn two() -> std::result::Result<i32, Box<dyn std::error::Error>> {
    let mut input = aoc_lib::read_input()?;

    let mut elves: Vec<i32> = input
        .split("\n\n")
        .map(|elf| {
            elf.split_terminator("\n")
                .fold(0, |acc, line| acc + line.parse::<i32>().unwrap())
        })
        .collect();

    elves.sort_unstable();
    return Ok(elves[elves.len() - 3..].iter().sum::<i32>());
}
