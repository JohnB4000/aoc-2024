use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day04;

impl Day for Day04 {
    type ParsedData = Vec<Vec<char>>;

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        Ok(
            BufReader::new(File::open(input_directory + "day04.txt").expect("File Error"))
                .lines()
                .map(|line| line.unwrap().chars().collect())
                .collect(),
        )
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let mut count = 0;

        for i in 0..input.len() as i32 {
            for j in 0..input[0].len() as i32 {
                if input[i as usize][j as usize] == 'X' {
                    for x in -1..2 as i32 {
                        for y in -1..2 as i32 {
                            if let Some(char) = get_char(input, i + x, j + y) {
                                if char == 'M' {
                                    if let Some(char) = get_char(input, i + 2 * x, j + 2 * y) {
                                        if char == 'A' {
                                            if let Some(char) =
                                                get_char(input, i + 3 * x, j + 3 * y)
                                            {
                                                if char == 'S' {
                                                    count += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        count
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let mut count = 0;

        for i in 0..input.len() as i32 {
            for j in 0..input[0].len() as i32 {
                if input[i as usize][j as usize] == 'A' {
                    if has_mas(input, i, j, &[(-1, -1), (1, 1)])
                        && has_mas(input, i, j, &[(1, -1), (-1, 1)])
                    {
                        count += 1
                    }
                }
            }
        }

        count
    }
}

fn get_char(input: &Vec<Vec<char>>, i: i32, j: i32) -> Option<char> {
    if i < 0 || j < 0 {
        return None;
    }

    input.get(i as usize)?.get(j as usize).copied()
}

fn has_mas(input: &Vec<Vec<char>>, i: i32, j: i32, xy: &[(i32, i32)]) -> bool {
    let mut previous: Option<char> = None;

    for (x, y) in xy {
        if let Some(current) = get_char(input, i + x, j + y) {
            match (current, previous) {
                (_, None) => previous = Some(current),
                ('S', Some('M')) | ('M', Some('S')) => return true,
                (_, _) => return false,
            }
        } else {
            return false;
        }
    }

    true
}
