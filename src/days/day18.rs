use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day18;

impl Day for Day18 {
    type ParsedData = String;

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let file = match File::open(input_directory + "day18.txt") {
            Ok(file) => file,
            Err(e) => return Err(e.to_string()),
        };
        let reader = BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(line) => todo!(),
                Err(e) => todo!(),
            }
        }

        Ok(String::new())
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        todo!()
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        todo!()
    }
}
