#![allow(unused)]

use std::{error::Error, result::Result};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Non-valid sign '{}'", value),
        }
    }
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    aoc_lib::print_result(&one()?, &two()?);
    Ok(())
}

fn one() -> Result<i32, Box<dyn Error>> {
    let mut input = aoc_lib::read_input()?;

    let score = input
        .split_terminator("\n")
        .map(|line| {
            let mut iter = line.split_whitespace();
            let oponent = Hand::from(iter.next().unwrap());
            let me = Hand::from(iter.next().unwrap());

            let outcome_points = match (&oponent, &me) {
                (Hand::Paper, Hand::Paper) => Outcome::Draw,
                (Hand::Paper, Hand::Rock) => Outcome::Loss,
                (Hand::Paper, Hand::Scissors) => Outcome::Win,
                (Hand::Rock, Hand::Paper) => Outcome::Win,
                (Hand::Rock, Hand::Rock) => Outcome::Draw,
                (Hand::Rock, Hand::Scissors) => Outcome::Loss,
                (Hand::Scissors, Hand::Paper) => Outcome::Loss,
                (Hand::Scissors, Hand::Rock) => Outcome::Win,
                (Hand::Scissors, Hand::Scissors) => Outcome::Draw,
                (_, _) => unreachable!(),
            } as i32;

            outcome_points + me as i32
        })
        .sum::<i32>();

    Ok(score)
}

fn two() -> Result<i32, Box<dyn Error>> {
    let mut input = aoc_lib::read_input()?;

    let score = input
        .split_terminator("\n")
        .map(|line| {
            let mut iter = line.split_terminator(" ");
            let oponent = Hand::from(iter.next().unwrap());
            let required_outcome = Outcome::from(iter.next().unwrap());

            let me = match required_outcome {
                Outcome::Draw => oponent,
                Outcome::Win => match oponent {
                    Hand::Paper => Hand::Scissors,
                    Hand::Rock => Hand::Paper,
                    Hand::Scissors => Hand::Rock,
                    _ => unreachable!(),
                },
                Outcome::Loss => match oponent {
                    Hand::Paper => Hand::Rock,
                    Hand::Rock => Hand::Scissors,
                    Hand::Scissors => Hand::Paper,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };

            required_outcome as i32 + me as i32
        })
        .sum::<i32>();

    Ok(score)
}
