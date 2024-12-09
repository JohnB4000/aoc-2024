use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day08;

impl Day for Day08 {
    type ParsedData = (Vec<(char, (i32, i32))>, i32, i32);

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let input =
            BufReader::new(File::open(input_directory + "day08.txt").expect("File Error")).lines();

        let mut rows = 0;
        let mut cols = 0;

        let mut coords = Vec::new();

        for (row, line) in input.enumerate() {
            let line = line.unwrap();
            rows += 1;
            cols = 0;

            for (col, c) in line.chars().enumerate() {
                cols += 1;
                if c != '.' {
                    coords.push((c, (row as i32, col as i32)));
                }
            }
        }

        Ok((coords, rows - 1, cols - 1 as i32))
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let (coords, rows, cols) = input;
        let coords: HashSet<(i32, i32)> = coords
            .iter()
            .flat_map(|&(c1, (x1, y1))| {
                coords.iter().filter_map(move |&(c2, (x2, y2))| {
                    if c1 != c2 || x1 == x2 && y1 == y2 {
                        return None;
                    }

                    let (nx, ny) = (x1 + x1 - x2, y1 + y1 - y2);

                    if nx < 0 || nx > *rows || ny < 0 || ny > *cols {
                        return None;
                    }

                    Some((nx, ny))
                })
            })
            .collect();

        coords.len() as i32
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let (coords, rows, cols) = input;
        let coords: HashSet<(i32, i32)> = coords
            .iter()
            .flat_map(|&(c1, (x1, y1))| {
                coords
                    .iter()
                    .filter_map(move |&(c2, (x2, y2))| {
                        if c1 != c2 || x1 == x2 && y1 == y2 {
                            return None;
                        }

                        let mut nodes = Vec::new();
                        nodes.push((x1, y1));

                        let (xoff, yoff) = ((x1 - x2), (y1 - y2));

                        let (mut nx, mut ny) = (x1, y1);

                        loop {
                            nx += xoff;
                            ny += yoff;

                            if nx < 0 || nx > *rows || ny < 0 || ny > *cols {
                                break;
                            }

                            nodes.push((nx, ny));
                        }

                        Some(nodes)
                    })
                    .flatten()
            })
            .collect();

        coords.len() as i32
    }
}
