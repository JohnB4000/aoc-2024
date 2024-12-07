use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

#[derive(Clone, Debug)]
enum Operators {
    Add,
    Mul,
    Concat,
}

#[derive(Clone, Debug)]
enum EquationSegment {
    Value(u64),
    Operator(Operators),
}

pub struct Day07;

impl Day for Day07 {
    type ParsedData = Vec<(u64, Vec<u64>)>;

    type Part1Result = u64;

    type Part2Result = u64;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        Ok(
            BufReader::new(File::open(input_directory + "day07.txt").expect("File Error"))
                .lines()
                .filter_map(|line| {
                    line.ok()?.split_once(": ").map(|(result, values)| {
                        (
                            result.parse().unwrap(),
                            values
                                .split_whitespace()
                                .map(|e| e.parse().unwrap())
                                .collect(),
                        )
                    })
                })
                .collect(),
        )
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        input
            .iter()
            .map(|(result, values)| {
                evaluate_equation(result, values, &[Operators::Add, Operators::Mul])
            })
            .sum()
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        input
            .iter()
            .map(|(result, values)| {
                evaluate_equation(
                    result,
                    values,
                    &[Operators::Add, Operators::Mul, Operators::Concat],
                )
            })
            .sum()
    }
}

fn evaluate_equation(result: &u64, values: &Vec<u64>, operators: &[Operators]) -> u64 {
    let mut equations: Vec<Vec<EquationSegment>> = Vec::new();
    equations.push(Vec::new());

    values.iter().for_each(|value| {
        let temp = equations.clone();
        equations.clear();
        for operator in operators {
            let mut new_temp = temp.clone();
            new_temp
                .iter_mut()
                .for_each(|equation| equation.push(EquationSegment::Value(*value)));

            new_temp
                .iter_mut()
                .for_each(|equation| equation.push(EquationSegment::Operator(operator.clone())));
            equations.extend(new_temp);
        }
    });

    for equation in equations {
        let mut operator = None;
        if let EquationSegment::Value(mut value) = equation[0] {
            for i in 1..equation.len() {
                match &equation[i] {
                    EquationSegment::Value(right) => match operator {
                        Some(Operators::Add) => value += right,
                        Some(Operators::Mul) => value *= right,
                        Some(Operators::Concat) => {
                            value = (value * 10_u64.pow(right.to_string().len() as u32)) + right
                        }
                        x => todo!("Missing {:?}", x),
                    },
                    EquationSegment::Operator(operators) => operator = Some(operators.clone()),
                }
            }
            if value == *result {
                return *result;
            }
        }
    }

    0
}
