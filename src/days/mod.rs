use std::fmt::Display;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub trait Day {
    type ParsedData;
    type Part1Result: Display;
    type Part2Result: Display;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String>;
    fn part_1(input: &Self::ParsedData) -> Self::Part1Result;
    fn part_2(input: &Self::ParsedData) -> Self::Part2Result;

    fn run(input_directory: String) {
        match Self::parse(input_directory) {
            Err(e) => println!("Error occured during parsing input: {:?}", e),
            Ok(parsed_data) => {
                println!("Running Part 1");
                println!("Solution for part 1: {}", Self::part_1(&parsed_data));
                println!("Solution for part 2: {}", Self::part_2(&parsed_data));
            }
        }
    }
}
