use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day10;

impl Day for Day10 {
    type ParsedData = Vec<Vec<u32>>;

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        Ok(
            BufReader::new(File::open(input_directory + "day10.txt").expect("File Error"))
                .lines()
                .map(|line| {
                    line.unwrap()
                        .chars()
                        .map(|value| value.to_digit(10).unwrap())
                        .collect()
                })
                .collect(),
        )
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let mut count = 0;

        for (row, arr) in input.iter().enumerate() {
            for (col, value) in arr.iter().enumerate() {
                if *value != 0 {
                    continue;
                }

                let mut queue = vec![(row, col)];
                let mut visited: HashSet<(usize, usize)> = HashSet::new();

                while let Some((row, col)) = queue.pop() {
                    if visited.contains(&(row, col)) {
                        continue;
                    }

                    visited.insert((row, col));

                    if input[row][col] == 9 {
                        count += 1;
                    }

                    for (neighbour_row, neighbour_col) in get_neighbours(input, row, col) {
                        if input[row][col] + 1 == input[neighbour_row][neighbour_col] {
                            queue.push((neighbour_row, neighbour_col));
                        }
                    }
                }
            }
        }

        count
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let mut count = 0;

        for (row, arr) in input.iter().enumerate() {
            for (col, value) in arr.iter().enumerate() {
                if *value != 0 {
                    continue;
                }

                let mut queue = vec![(row, col)];

                while let Some((row, col)) = queue.pop() {
                    if input[row][col] == 9 {
                        count += 1;
                    }

                    for (neighbour_row, neighbour_col) in get_neighbours(input, row, col) {
                        if input[row][col] + 1 == input[neighbour_row][neighbour_col] {
                            queue.push((neighbour_row, neighbour_col));
                        }
                    }
                }
            }
        }

        count
    }
}

fn get_neighbours(input: &Vec<Vec<u32>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    if row + 1 < input.len() {
        neighbours.push((row + 1, col));
    }
    if col + 1 < input[0].len() {
        neighbours.push((row, col + 1));
    }

    if row > 0 {
        neighbours.push((row - 1, col));
    }
    if col > 0 {
        neighbours.push((row, col - 1));
    }

    neighbours
}
