use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use super::Day;

pub struct Day03;

#[derive(Debug)]
enum Code {
    DO,
    DONT,
    MUL(i32, i32),
}

impl Day for Day03 {
    type ParsedData = String;

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        Ok(
            BufReader::new(File::open(input_directory + "day03.txt").expect("File Error"))
                .lines()
                .map(|line| line.unwrap())
                .reduce(|acc, line| acc + &line)
                .unwrap(),
        )
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        let mut instructions = vec![];
        for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
            instructions.push((left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()));
        }

        instructions.iter().map(|(left, right)| left * right).sum()
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(do\(\))|(don't\(\))").unwrap();
        let instructions: Vec<Code> = re
            .captures_iter(input)
            .filter_map(|c| {
                if let Some(left) = c.get(1) {
                    Some(Code::MUL(
                        left.as_str().parse().unwrap(),
                        c.get(2).unwrap().as_str().parse().unwrap(),
                    ))
                } else if let Some(_) = c.get(3) {
                    Some(Code::DO)
                } else {
                    Some(Code::DONT)
                }
            })
            .collect();

        let mut sum = 0;
        let mut skip = false;

        instructions
            .iter()
            .for_each(|instruction| match instruction {
                Code::MUL(left, right) => {
                    if !skip {
                        sum += left * right
                    }
                }
                Code::DONT => skip = true,
                Code::DO => skip = false,
            });

        sum
    }
}
