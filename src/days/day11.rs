use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day11;

impl Day for Day11 {
    type ParsedData = Vec<u64>;

    type Part1Result = usize;

    type Part2Result = usize;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        Ok(
            BufReader::new(File::open(input_directory + "day11.txt").expect("File Error"))
                .lines()
                .flat_map(|line| {
                    line.unwrap()
                        .split_whitespace()
                        .map(|stone| stone.parse().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect(),
        )
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let process = |input: Vec<u64>| -> Vec<u64> {
            input
                .iter()
                .flat_map(|stone| match stone {
                    0 => vec![1],
                    _ if stone.to_string().len() % 2 == 0 => {
                        let mut temp = stone.to_string().clone();
                        let other = temp.split_off(temp.len() / 2);
                        vec![temp.parse().unwrap(), other.parse().unwrap()]
                    }
                    _ => vec![stone * 2024],
                })
                .collect::<Vec<u64>>()
        };

        let mut input = input.clone();

        for _ in 0..25 {
            input = process(input);
        }

        input.len()
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        // let process = |input: Vec<u64>| -> Vec<u64> {
        //     input
        //         .par_iter()
        //         .flat_map(|stone| match stone {
        //             0 => vec![1],
        //             _ if stone.to_string().len() % 2 == 0 => {
        //                 let mut temp = stone.to_string().clone();
        //                 let other = temp.split_off(temp.len() / 2);
        //                 vec![temp.parse().unwrap(), other.parse().unwrap()]
        //             }
        //             _ => vec![stone * 2024],
        //         })
        //         .collect::<Vec<u64>>()
        // };
        //
        // let mut input = input.clone();
        //
        // for i in 0..75 {
        //     input = process(input);
        //     dbg!(i);
        // }

        input.len()
    }
}
