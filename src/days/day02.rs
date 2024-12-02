use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

pub struct Day02;

impl Day for Day02 {
    type ParsedData = Vec<Vec<i32>>;

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let reader = BufReader::new(File::open(input_directory + "day02.txt").expect("File Error"));

        let mut data = Vec::new();

        for line in reader.lines() {
            let mut row = Vec::new();
            let line = line.expect("Expected a row");
            let mut line = line.split_whitespace();
            loop {
                match line.next() {
                    Some(value) => row.push(value.parse().unwrap()),
                    None => break,
                }
            }
            data.push(row)
        }

        Ok(data)
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let mut safe_count = 0;
        for row in input {
            let mut ascending = false;
            let mut descending = false;
            let mut same_value = false;
            let mut diff_safe = true;
            for i in 0..(row.len() - 1) {
                if row[i] < row[i + 1] {
                    ascending = true;
                } else if row[i] > row[i + 1] {
                    descending = true;
                } else {
                    same_value = true
                }

                let diff = (row[i] - row[i + 1]).abs();
                if diff < 1 || diff > 3 {
                    diff_safe = false;
                }
            }
            if (ascending && !descending || !ascending && descending) && !same_value && diff_safe {
                safe_count += 1;
            }
        }
        safe_count
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let mut safe_count = 0;
        for row in input {
            let subsets = get_subsets(row.to_vec());
            let mut safe = false;
            for subset in subsets {
                let mut ascending = false;
                let mut descending = false;
                let mut same_value = false;
                let mut diff_safe = true;
                if subset.is_empty() {
                    continue;
                }
                for i in 0..(subset.len() - 1) {
                    if subset[i] < subset[i + 1] {
                        ascending = true;
                    } else if subset[i] > subset[i + 1] {
                        descending = true;
                    } else {
                        same_value = true
                    }

                    let diff = (subset[i] - subset[i + 1]).abs();
                    if diff < 1 || diff > 3 {
                        diff_safe = false;
                    }
                }
                if (ascending && !descending || !ascending && descending)
                    && !same_value
                    && diff_safe
                {
                    safe = true;
                }
            }
            if safe {
                safe_count += 1;
            }
        }
        safe_count
    }
}

fn get_subsets(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut subsets = Vec::new();
    subsets.push(set.clone());

    for i in 0..set.len() {
        let mut subset = set.clone();
        subset.remove(i);
        subsets.push(subset);
    }

    subsets
}
