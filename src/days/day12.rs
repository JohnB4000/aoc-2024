use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day12;

impl Day for Day12 {
    type ParsedData = Vec<Vec<char>>;

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        Ok(
            BufReader::new(File::open(input_directory + "day12.txt").expect("File Error"))
                .lines()
                .map(|line| line.unwrap().chars().collect())
                .collect(),
        )
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let mut price = 0;

        let mut visited = HashSet::new();

        for (row, arr) in input.iter().enumerate() {
            for (col, value) in arr.iter().enumerate() {
                if visited.contains(&(row, col)) {
                    continue;
                }

                let mut area = 0;
                let mut perimeter = 0;

                let mut queue = vec![(row, col)];

                while let Some((row, col)) = queue.pop() {
                    if visited.contains(&(row, col)) {
                        continue;
                    }

                    visited.insert((row, col));

                    area += 1;

                    for neighbour in get_neighbours(input, row, col) {
                        match neighbour {
                            Some((neighbour_row, neighbour_col)) => {
                                if input[row][col] == input[neighbour_row][neighbour_col] {
                                    queue.push((neighbour_row, neighbour_col));
                                } else {
                                    perimeter += 1;
                                }
                            }
                            None => perimeter += 1,
                        }
                    }
                }

                price += area * perimeter;
            }
        }

        price
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        todo!()
    }
}

fn get_neighbours(input: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<Option<(usize, usize)>> {
    let mut neighbours = Vec::new();

    if row + 1 < input.len() {
        neighbours.push(Some((row + 1, col)));
    } else {
        neighbours.push(None);
    }
    if col + 1 < input[0].len() {
        neighbours.push(Some((row, col + 1)));
    } else {
        neighbours.push(None);
    }

    if row > 0 {
        neighbours.push(Some((row - 1, col)));
    } else {
        neighbours.push(None);
    }

    if col > 0 {
        neighbours.push(Some((row, col - 1)));
    } else {
        neighbours.push(None);
    }

    neighbours
}
