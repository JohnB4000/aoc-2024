use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use super::Day;

#[derive(Debug, Clone)]
pub enum Operation {
    AND,
    OR,
    XOR,
}

pub struct Day24;

impl Day for Day24 {
    type ParsedData = (
        HashMap<String, u64>,
        Vec<(String, Operation, String, String)>,
    );

    type Part1Result = u64;

    type Part2Result = u64;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let mut state = HashMap::new();
        let mut instructions = Vec::new();

        let re = Regex::new(r"(?:(\w+): (\d))|(?:(\w+) (XOR|OR|AND) (\w+) -> (\w+))").unwrap();
        BufReader::new(File::open(input_directory + "day24.txt").expect("File Error"))
            .lines()
            .for_each(|line| {
                re.captures_iter(&line.unwrap()).for_each(|c| {
                    if let Some(reg) = c.get(1) {
                        state.insert(
                            reg.as_str().to_string(),
                            c.get(2).unwrap().as_str().parse().unwrap(),
                        );
                    } else {
                        instructions.push((
                            c.get(3).unwrap().as_str().to_string(),
                            match c.get(4).unwrap().as_str() {
                                "AND" => Operation::AND,
                                "OR" => Operation::OR,
                                "XOR" => Operation::XOR,
                                _ => todo!(),
                            },
                            c.get(5).unwrap().as_str().to_string(),
                            c.get(6).unwrap().as_str().to_string(),
                        ));
                    }
                })
            });

        Ok((state, instructions))
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let (mut state, mut instructions) = input.clone();

        while instructions.len() > 0 {
            instructions = instructions
                .iter()
                .filter_map(|(left, op, right, result)| {
                    if !state.contains_key(left) || !state.contains_key(right) {
                        Some((
                            left.to_string(),
                            op.clone(),
                            right.to_string(),
                            result.to_string(),
                        ))
                    } else {
                        state.insert(
                            result.to_string(),
                            match op {
                                Operation::AND => {
                                    state.get(left).unwrap() & state.get(right).unwrap()
                                }
                                Operation::OR => {
                                    state.get(left).unwrap() | state.get(right).unwrap()
                                }
                                Operation::XOR => {
                                    state.get(left).unwrap() ^ state.get(right).unwrap()
                                }
                            },
                        );
                        None
                    }
                })
                .collect();
        }

        let result = state
            .iter()
            .filter(|(key, value)| key.chars().next().unwrap() == 'z')
            .collect::<Vec<(&String, &u64)>>();

        let mut number = vec![0; result.len()];

        result.iter().for_each(|(key, value)| {
            let (_, index) = key.split_at(1);
            number[index.parse::<usize>().unwrap()] = **value;
        });

        number.reverse();
        let number: String = number.iter().map(|num| num.to_string()).collect();
        u64::from_str_radix(&number, 2).unwrap()
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        todo!()
    }
}
