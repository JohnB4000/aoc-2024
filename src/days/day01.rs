use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day01;

impl Day for Day01 {
    type ParsedData = (Vec<i32>, Vec<i32>);

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let reader = BufReader::new(File::open(input_directory + "day01.txt").expect("File Error"));

        let mut left_list = Vec::new();
        let mut right_list = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Error reading line");
            let mut data = line.split_whitespace();
            left_list.push(
                data.next()
                    .expect("Expected a number")
                    .parse()
                    .expect("Expected a number"),
            );
            right_list.push(
                data.next()
                    .expect("Expected a number")
                    .parse()
                    .expect("Expected a number"),
            );
        }

        Ok((left_list, right_list))
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let (mut left_list, mut right_list) = input.clone();
        left_list.sort();
        right_list.sort();

        left_list
            .iter()
            .zip(right_list.iter())
            .map(|(left_item, right_item)| (left_item - right_item).abs())
            .sum()
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let (left_list, right_list) = input.clone();
        let mut occurrances = HashMap::new();
        for item in right_list {
            *occurrances.entry(item).or_insert(0) += 1;
        }

        left_list
            .iter()
            .map(|item| item * occurrances.get(item).unwrap_or(&0))
            .sum()
    }
}
